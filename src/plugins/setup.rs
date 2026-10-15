// plugins/setup.rs
use bevy::prelude::*;
use crate::StartupSet;

pub struct SetupPlugin;
use crate::components::*;
use crate::resources::*;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            init_entity_grid.in_set(StartupSet::SpawnGrid),
            randomly_init_live.in_set(StartupSet::SeedLife),
        ));
    }
}

fn randomly_init_live(
    mut query: Query<(&mut CellState, &mut MeshMaterial2d<ColorMaterial>)>,
    materials: Res<CellResource>,
) {
    for (mut cell_state, mut mat) in &mut query {
        if !cell_state.current && rand::random::<f32>() < 0.1 {
            cell_state.current = true;
            mat.0 = materials.alive.clone();
        }
    }
}

fn init_entity_grid(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    conf: Res<Config>,
) {
    let alive_handle = materials.add(Color::WHITE);
    let dead_handle = materials.add(Color::NONE);
    let cell_mesh = meshes.add(Rectangle::new(conf.cell_size, conf.cell_size));

    let mut sorted_by_ids = Vec::with_capacity(
        conf.grid.size()
    );

    command.insert_resource(CellResource {
        alive: alive_handle.clone(),
        dead: dead_handle.clone(),
        mesh: cell_mesh.clone()
    });

    command.insert_resource(CellWeights {
        weights: vec![0; conf.arena_xy_resolution()],
        // Сразу создаем вектор нужного размера, заполненный нулями
    });

    // Камера
    command.spawn((
        Camera2d,
        Transform::from_xyz(
            conf.arena_half_x_len_pad(),
            conf.arena_half_y_len_pad(),
            1000.0
        ),
    ));

    let mut cell_idx = 0;
        
    // Спавн клеток
    for pos_x in 0..conf.arena_x_resolution() {
        for pos_y in 0..conf.arena_x_resolution() {
            let e = command.spawn(((
                CellState { current: false, previous: false },
                Neighbors(Vec::with_capacity(
                    conf.grid.count_observed_cells(conf.observ_size)
                )),
                CellID(cell_idx),
                Position(Vec2 {
                        x: pos_x as f32,
                        y: pos_y as f32,
                })
            ),
                Mesh2d(cell_mesh.clone()),
                MeshMaterial2d(dead_handle.clone()), 
                Transform::from_xyz(
                    pos_x as f32 * conf.cell_size * conf.cell_offset,
                    pos_y as f32 * conf.cell_size * conf.cell_offset,
                    0.0,
                ),
            ));

            // Ожидается, что порядок не будет изменен, и Entity = cell_idx
            // Не знаю, допустимо ли такое решение, или есть лучше, пока оставлю так
            sorted_by_ids.push((e.id(), cell_idx));

            cell_idx += 1;
        }
    }

    command.insert_resource(SortedCellsByIDS(sorted_by_ids));
}