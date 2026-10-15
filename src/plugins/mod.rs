// plugins/mod.rs

use bevy::prelude::*;
use crate::setup::SetupPlugin;
use crate::neighbors::NeighborsPlugin;
use crate::simulation::SimulationPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StartupSet {
    SpawnGrid,
    ConnectNeighbors,
    SeedLife,
    FirstActualization,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimSet {
    CalcWeights,
    ApplyRules,
    Actualize,
}

pub mod setup;
pub mod neighbors;
pub mod simulation;
pub mod debug;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, (
            StartupSet::SpawnGrid,
            StartupSet::ConnectNeighbors,
            StartupSet::SeedLife,
            StartupSet::FirstActualization,
        ).chain())
        .add_plugins((SetupPlugin, NeighborsPlugin, SimulationPlugin));
    }
}