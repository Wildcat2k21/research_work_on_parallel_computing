//resources

use bevy::prelude::*;
use crate::components::*;

// Ресурсы
#[derive(Resource)]
pub struct Config {
    pub cell_size: f32,
    pub cell_offset: f32,
    pub observ_size: usize,
    pub grid: Grid,
}

#[derive(Resource)]
pub struct Grid {
    pub x_amount: usize,
    pub y_amount: usize,
}

// Ресурсы
#[derive(Resource, Debug, Clone)]
pub struct CellWeights {
    pub weights: Vec<i32>, // Индекс = CellID. Значение = вес.
}

#[derive(Resource, Clone)]
pub struct NextIteration(pub Vec<(Entity, usize)>);

#[derive(Resource)]
pub struct CellResource {
    pub alive: Handle<ColorMaterial>,
    pub dead: Handle<ColorMaterial>,
    pub mesh: Handle<Mesh>
}

#[derive(Resource)]
pub struct SortedCellsByIDS(pub Vec<(Entity, usize)>);

#[derive(Clone, Copy, PartialEq)]
pub enum UseOptimization {
    CacheConnections //Пока будет только кэширование
}

pub struct DemoOptimize(pub Vec<UseOptimization>);

#[derive(Resource)]
pub struct DemoConfig {
    pub optimizations: DemoOptimize,
    // Если будут другие оптимизации
}

// resources.rs (рядом с DemoConfig)
impl DemoConfig {
    pub fn has(&self, opt: UseOptimization) -> bool {
        self.optimizations.0.contains(&opt)
    }
}

pub fn optimization_enabled(
    opt: UseOptimization,
) -> impl Fn(Res<DemoConfig>) -> bool + Clone {
    move |cfg: Res<DemoConfig>| cfg.has(opt)
}

impl Grid {
    pub fn new(
        x_amount: usize,
        y_amount: usize,
    ) -> Grid {
        Grid {
            x_amount,
            y_amount,
        }
    }

    pub fn add_cells(&self, cell: CellGrid, other: CellGrid) -> Option<CellGrid> {
        let new_cell = CellGrid::new(
            cell.x + other.x,
            cell.y + other.y
        );
        
        if new_cell.x < self.x_amount
        && new_cell.y < self.y_amount {
            Some(new_cell)
        }
        else {
            None
        }
    }

    pub fn cell_add_index(&self, cell: CellGrid, index: usize) -> Option<CellGrid> {
        match self.cid_to_gid(index) {
            Some(g) => {
                self.add_cells(cell, g)
            },
            None => None
        }
    }

    pub fn index_add_index(&self, index_1: usize, index_2: usize) -> Option<usize>{
        if index_1 + index_2 < self.size() {
            Some(index_1 + index_2)
        }
        else {
            None
        }
    }

    pub fn index_add_cell(&self, index: usize, cell: CellGrid) -> Option<CellGrid> {
        match self.cid_to_gid(index) {
            Some(g) => {
                self.add_cells(cell, g)
            },
            None => None
        }
    }

    pub fn count_observed_cells(&self, observ_size: usize) -> usize {
        (2 * observ_size - 1) * (2 * observ_size - 1)
    }

    pub fn cell_is_observed(&self, cell_1: &CellGrid, cell_2: &CellGrid, observ_size: usize) -> bool {
        let offset_x_mod = i32::abs(cell_1.x as i32 - cell_2.x as i32) as usize;
        let offset_y_mod = i32::abs(cell_1.y as i32 - cell_2.y as i32) as usize;

        if offset_x_mod > self.x_amount
        || offset_y_mod > self.y_amount {
            false
        }
        else {
            offset_x_mod < observ_size
            && offset_y_mod < observ_size
        }
    }

    pub fn size(&self) -> usize {
        self.x_amount * self.y_amount
    }

    pub fn cid_to_gid(
        &self, cell_id: usize,
    ) -> Option<CellGrid> {
        if cell_id < self.size() {
            Some(
                CellGrid::new(
                    cell_id % self.x_amount,
                    cell_id / self.x_amount
                )
            )
        }
        else {
            None
        }
    }

    pub fn gid_to_cid(&self, CellGrid { x, y }: CellGrid) -> Option<usize> {
        if x < self.x_amount
        && y < self.y_amount {
            Some(x + y * self.x_amount)
        }
        else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a SortedCellsByIDS {
    type Item = &'a (Entity, usize);
    type IntoIter = std::slice::Iter<'a, (Entity, usize)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Config {
    pub fn new(
        cell_size: f32, 
        cell_offset: f32,
        observ_size: usize,
        grid: Grid
    ) -> Config {
        Config {
            cell_size,
            cell_offset,
            observ_size,
            grid
        }
    }

    pub fn arena_xy_resolution(&self) -> usize {
        self.grid.size()
    }

    pub fn arena_x_resolution(&self) -> usize {
        self.grid.x_amount
    }

    pub fn arena_y_resolution(&self) -> usize {
        self.grid.y_amount
    }

    pub fn arena_x_len_pad(&self) -> f32 {
        self.grid.x_amount as f32 * self.cell_offset * self.cell_size
    }

    pub fn arena_y_len_pad(&self) -> f32 {
        self.grid.y_amount as f32 * self.cell_offset * self.cell_size
    }

    pub fn arena_xy_size_pad(&self) -> f32 {
        self.grid.size() as f32 * self.cell_offset * self.cell_size
    }

    pub fn arena_half_x_len_pad(&self) -> f32 {
        self.arena_x_len_pad() / 2.0
    }

    pub fn arena_half_y_len_pad(&self) -> f32 {
        self.arena_y_len_pad() / 2.0
    }
}
