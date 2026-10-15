use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::neighbors_cache::load_or_compute_connections;
use crate::StartupSet;

pub struct NeighborsPlugin;

impl Plugin for NeighborsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            connect_cells
                .run_if(not(optimization_enabled(UseOptimization::CacheConnections))),
            connect_cells_cached
                .run_if(optimization_enabled(UseOptimization::CacheConnections)),
        ).in_set(StartupSet::ConnectNeighbors));
    }
}

fn connect_cells(
    sorted_cells: Res<SortedCellsByIDS>,
    conf: Res<Config>,
    mut cell_query: Query<&mut Neighbors>
) {
    let sorted_selected_ids: Vec<_> = sorted_cells
    .0.iter().map(|(_, ids)| *ids)
    .collect();

    for (cell_entity, cell_id) in &sorted_cells.0 {
        let seria = Seria::new(
            &sorted_selected_ids,
            &conf.grid,
            *cell_id,
            conf.observ_size + 1
        );

        let mut nbors = vec![(*cell_entity, *cell_id)];
        let mut cell = cell_query.get_mut(*cell_entity).unwrap();

        for obs_cell in seria {
            if obs_cell != cell_id {

                let (entity_for_obs_cell, _) = &sorted_cells
                    .0
                    .iter()
                    .find(|(_, cell_id)| cell_id == obs_cell)
                    .unwrap();

                nbors.push((*entity_for_obs_cell, *obs_cell));
            }
        }

        (*cell).0.extend_from_slice(&nbors);

    }
}


fn connect_cells_cached(
    sorted_cells: Res<SortedCellsByIDS>,
    conf: Res<Config>,
    mut cell_query: Query<&mut Neighbors>,
) {
    println!("🔗 Построение графа соседей...");
    
    // 1. Получаем кэш (или генерируем его с прогресс-баром)
    let cached_connections = load_or_compute_connections(&conf, &sorted_cells);

    // 2. Создаем быстрый словарь для маппинга CellID -> Entity
    // (Вектор быстрее HashMap, так как индексы идут от 0 до N-1)
    let mut id_to_entity: Vec<Entity> = vec![Entity::PLACEHOLDER; sorted_cells.0.len()];
    for &(entity, cell_id) in &sorted_cells.0 {
        id_to_entity[cell_id] = entity;
    }

    // 3. Применяем кэшированные данные к реальным сущностям
    for (cell_entity, cell_id) in &sorted_cells.0 {
        let mut cell = cell_query.get_mut(*cell_entity).unwrap();
        
        // Берем закэшированные CellID для этой клетки
        let cached_nbors = &cached_connections[*cell_id];
        
        // Преобразуем их в (Entity, CellID) кортежи
        let mut nbors = Vec::with_capacity(cached_nbors.len());
        for &cid in cached_nbors {
            nbors.push((id_to_entity[cid], cid));
        }
        
        (*cell).0 = nbors;
    }
    println!("✅ Граф соседей построен!");
}