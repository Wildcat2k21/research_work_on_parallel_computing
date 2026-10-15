mod components;
mod resources;
mod neighbors_cache;
mod plugins;

use resources::*;
use plugins::*;
use debug::DebugPlugin;

use bevy::{
    DefaultPlugins,
    app::App,
};

fn main() {
    let conf = Config::new(5.0, 1.0, 2, Grid::new(400, 400));
    let demo_conf = DemoConfig {
        optimizations: DemoOptimize(vec![UseOptimization::CacheConnections]),
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(conf)
        .insert_resource(demo_conf)
        .insert_resource(NextIteration(Vec::new()))
        .add_plugins(GameOfLifePlugin)
        // .add_plugins(DebugPlugin)
        .run();
}