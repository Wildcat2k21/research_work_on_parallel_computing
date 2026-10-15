// plugins/simulation.rs
use std::collections::HashSet;
use crate::SimSet;

pub struct SimulationPlugin;

// plugins/setup.rs
use bevy::prelude::*;
use crate::StartupSet;

use crate::components::*;
use crate::resources::*;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,
            initial_cell_actualization.in_set(StartupSet::FirstActualization),
        )
        .configure_sets(Update, (
            SimSet::CalcWeights,
            SimSet::ApplyRules,
            SimSet::Actualize,
        ).chain())
        .add_systems(Update, (
            calc_cell_weight.in_set(SimSet::CalcWeights),
            mark_live_cells.in_set(SimSet::ApplyRules),
            actualizate_cells.in_set(SimSet::Actualize),
        ));
    }
}

// В calc_cell_weight:
fn calc_cell_weight(
    cell_query: Query<(&Neighbors, &CellState, &CellID)>, // Добавили &CellID в Query!
    mut weighted_cells: ResMut<CellWeights>,
    actual_cells: Res<NextIteration>
) {
    // Очищать не нужно, мы будем перезаписывать значения для активных клеток,
    // а для неактивных вес и так останется 0 (или мы можем обнулять только нужные, но для простоты пока оставим так).
    // Но лучше сделать так:
    
    for (actual_entity, _) in &actual_cells.0 {
        let (neighbors, _, cell_id) = cell_query.get(*actual_entity).unwrap();

        let mut live_amount = 0;
        for (neighbor_entity, _) in neighbors.get_extended() {
            let (_, neighbor_state, _) = cell_query.get(*neighbor_entity).unwrap();
            if neighbor_state.current {
                live_amount += 1;
            }
        }

        // МАГИЯ O(1): Прямой доступ по индексу CellID!
        weighted_cells.weights[cell_id.0] = live_amount as i32;
    }
}

fn mark_live_cells(
    mut cell_query: Query<(&mut CellState, &CellID, &mut MeshMaterial2d<ColorMaterial>)>,
    materials: Res<CellResource>,
    weighted_cells: Res<CellWeights>,
    actual_cells: Res<NextIteration>, 
) {
    for (entity, _) in &actual_cells.0 {
        let (mut state, cid, mut mat) = cell_query.get_mut(*entity).unwrap();

        // 1. Фиксируем состояние
        let was_alive = state.current;
        state.previous = state.current;

        // 2. Прямое индексирование (гарантированно безопасно, так как cid.0 < weights.len())
        let w = weighted_cells.weights[cid.0];

        // 3. Правила Conway (B3 / S23)
        let neighbors_count = if was_alive { w - 1 } else { w };

        let next_state = if was_alive {
            neighbors_count == 2 || neighbors_count == 3
        } else {
            neighbors_count == 3
        };

        // 4. Применяем изменения
        if state.current != next_state {
            state.current = next_state;
            if next_state {
                mat.0 = materials.alive.clone();
            } else {
                mat.0 = materials.dead.clone();
            }
        }
    }
}

fn actualizate_cells (
    mut next_iter_res: ResMut<NextIteration>,
    mut cell_weights: ResMut<CellWeights>, // Убираем mut, мы будем работать с локальной копией или изменим логику
    cell_query: Query<(&CellState, &Neighbors, &CellID)>, // Добавили &CellID
) {
    let mut sub_elems = next_iter_res.0.clone();
    let mut sum_elems = Vec::new();

    for (entity, _) in &next_iter_res.0 {
        let (state, neighbors, _) = cell_query.get(*entity).unwrap();

        if state.current != state.previous {
            if state.current { // Клетка ожила
                sum_elems.extend_from_slice(neighbors.get_extended());
            } else { // Клетка умерла
                
                // Итерируемся по соседям мертвой клетки
                for (_, neighbor_cid) in neighbors.get_extended() {
                    
                    // МАГИЯ O(1): Мгновенно находим вес соседа по его CellID!
                    let weight_ref = &mut cell_weights.weights[*neighbor_cid];
                    *weight_ref -= 1;

                    // Если вес стал <= 0, удаляем из списка актуализации
                    if *weight_ref <= 0 {
                        sub_elems.retain(|(_, cid)| *cid != *neighbor_cid);
                    }
                }
            }
        }
    }

    sub_elems.extend_from_slice(&sum_elems);

    let mut seen_cids = HashSet::new();
    sub_elems.retain(|(_, cid)| seen_cids.insert(*cid));

    *next_iter_res = NextIteration(sub_elems);
    
    // Сохраняем обновленные веса обратно в ресурс (опционально, если они нужны где-то еще)
    // Но для следующей итерации calc_cell_weight всё равно их пересчитает.
}

// Первичная актуализация значений
fn initial_cell_actualization(
    mut next_iter_res: ResMut<NextIteration>,
    cell_query: Query<(&Neighbors, &CellState, &CellID)>,
) {
    let mut result = Vec::new();

    for (neightbors, state, _) in cell_query {
        if  state.current { //true is alive
            result.extend_from_slice(&neightbors.get_extended());
        }
    }
    
    let mut seen_cids = HashSet::new();
    result.retain(|(_, cid)| seen_cids.insert(*cid));
    *next_iter_res = NextIteration(result);
}
