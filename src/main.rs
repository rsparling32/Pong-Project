use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod ball;
mod playfield;
mod gamemanager;
mod paddle;
mod scoreboard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(ball::BallPlugin)
        .add_plugin(playfield::PlayfieldPlugin)
        .add_plugin(paddle::PaddlePlugin)
        .add_plugin(scoreboard::ScoreBoardPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
){
    commands
        .spawn_bundle(Camera2dBundle::default());
}

