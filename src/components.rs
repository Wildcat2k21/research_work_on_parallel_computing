//components.rs

use bevy::prelude::*;
use crate::resources::*;

#[derive(Component, Clone)]
pub struct Neighbors(pub Vec<(Entity, usize)>);

#[derive(Component, Clone, Copy, Debug)]
pub struct CellState {
    pub current: bool,  // true = Alive, false = Dead
    pub previous: bool, // Состояние на предыдущем кадре
}

#[derive(Component)]
pub struct CellID(pub usize);

#[derive(Component)]
pub struct Position(pub Vec2);

pub struct Seria<'a> {
    pub linear_present: &'a Vec<usize>,
    pub grid_present: &'a Grid,
    pub so: usize,
    pub sl: usize,
    pub row_index: usize,
    pub col_index: usize,
    pub seria_bounds: Option<(usize, usize, usize, usize)>
}

#[derive(Clone, Copy, Debug)]
pub struct CellGrid {
    pub x: usize,
    pub y: usize,
}

impl CellGrid {
    pub fn new(x: usize, y: usize) -> CellGrid{
        CellGrid {
            x,
            y
        }
    }
}

impl Neighbors {
    pub fn new(itself: (Entity, usize), mut other: Vec<(Entity, usize)>) -> Neighbors {
        other.insert(0, itself);
        Neighbors(other)
    }

    pub fn get_neighbors(&self) -> &[(Entity, usize)] {
        &self.0[1..]
    }

    pub fn get_extended(&self) -> &Vec<(Entity, usize)> {
        &self.0
    }
}

impl<'a> Seria<'a> {
    pub fn new(
        linear_present: &'a Vec<usize>,
        grid_present: &'a Grid,
        so: usize,
        sl: usize,
    ) -> Self {
        Seria {
            linear_present,
            grid_present,
            so,
            sl,
            row_index: 0,
            col_index: 0,
            seria_bounds: None
        }
    }

    pub fn seria_h_len(&mut self) -> usize {
        let (left, right, ..) = self.seria_bounds_offset_so();
        left + right + 1
    }

    pub fn seria_v_len(&mut self) -> usize {
        let (_, _, top, bottom) = self.seria_bounds_offset_so();
        top + bottom + 1
    }

    pub fn seria_bounds_offset_so(&mut self) ->
        (usize, usize, usize, usize)
    {
        if let Some(bounds) = self.seria_bounds {
            return bounds;
        }

        let sl = self.sl;
        let so = self.so;
        let grid = self.grid_present;
        let observ_size = (sl + 1) / 2;
        
        let so_cell_grid = self.grid_present
            .cid_to_gid(so)
            .unwrap();

        // Горизонтальная левая граница серии (Относительно центра серии so)
        let mut shl = 0; // от 0 до sl-1
        for chl in 1..((sl + 1) / 2) {
            if so >= chl
            && grid
                .cid_to_gid(so - chl)
                .unwrap().y == so_cell_grid.y
            && grid.cell_is_observed( // Проверка что so - chl обозрима
                &so_cell_grid,
                &grid
                    .cid_to_gid(so - chl)
                    .unwrap(),
                observ_size
            ) {
                shl += 1;
            }
            else {
                break;
            }
        }

        // Горизонтальная правая граница серии (Относительно центра серии so)
        let mut shr = 0; // от 0 до sl-1
        for chr in 1..((sl + 1) / 2) {

            let next_cid =
                so + chr;

            if next_cid < grid.size()
            && grid
                .cid_to_gid(next_cid)
                .unwrap().y == so_cell_grid.y
            && grid.cell_is_observed( // Проверка что shr + chr обозрима
                &so_cell_grid,
                &grid
                    .cid_to_gid(next_cid)
                    .unwrap(),
                observ_size
            ) {
                shr += 1;
            }
            else {
                break;
            }
        }

        // Вертикальная верхняя граница серии (Относительно центра серии so)
        let mut svt = 0; // от 0 до sl-1
        for cvt in 1..((sl + 1) / 2) { //от 1 до 3 вкл

            let next_cid =
                so + cvt * grid.x_amount;

            if next_cid < grid.size()
            && grid.cell_is_observed( // Проверка что svt + cvt обозрима
                &so_cell_grid,
                &grid
                    .cid_to_gid(next_cid)
                    .unwrap(),
                observ_size
            ) {
                svt += 1;
            }
            else {
                break;
            }
        }

        // Вертикальная нижняя граница серии (Относительно центра серии so)
        let mut svb = 0; // от 0 до sl-1
        for cvb in 1..((sl + 1) / 2) {
            if so >= cvb * grid.x_amount
            && grid.cell_is_observed( // Проверка что svb + cvt обозрима
                &so_cell_grid,
                &grid
                    .cid_to_gid(so - cvb * grid.x_amount)
                    .unwrap(),
                observ_size
            ) {
                svb += 1;
            }
            else {
                break;
            }
        }

        self.seria_bounds = Some((shl, shr, svt, svb));
        (shl, shr, svt, svb)
    }
}

impl<'a> Iterator for Seria<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        if !(self.col_index == 0
            && self.row_index == self.seria_v_len()) {
            let (
                left,
                _,
                _,
                bottom
            ) = self.seria_bounds_offset_so();
            
            // Центр серии
            let so = self
                .grid_present
                .cid_to_gid(self.so).unwrap();

            let seria_current = CellGrid::new(
                so.x - left + self.col_index,
                so.y - bottom + self.row_index,
            );

            let val = self.linear_present.get(
                self.grid_present.gid_to_cid(
                    seria_current,
                ).unwrap()
            ).unwrap();

            self.col_index += 1;

            if self.col_index == self.seria_h_len() {
                self.col_index = 0;
                self.row_index += 1;
            }

            Some(val)
        }
        else { 
            None
        }
    }
}


