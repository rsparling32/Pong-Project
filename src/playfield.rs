use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayfieldPlugin;

impl Plugin for PlayfieldPlugin{
    fn build(&self, app:&mut App){
        app
            .add_startup_system(setup)
    ;}
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("Barrier.png"),
            ..default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(500.,5.))
            .insert_bundle(TransformBundle{
                local: Transform{
                    translation: Vec3::new(-500.,450.,0.),
                    rotation: Quat::from_rotation_z(179.),
                    ..default()
                },
                global: default()
            })
            .insert(Friction::coefficient(0.0))
            .insert(Restitution::coefficient(1.0))
            .insert(Ccd::enabled());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("Barrier.png"),
            ..default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(500.,5.))
            .insert_bundle(TransformBundle{
                local: Transform{
                    translation: Vec3::new(500.,450.,0.),
                    rotation: Quat::from_rotation_z(-179.),
                    ..default()
                },
                global: default()
            })
            .insert(Friction::coefficient(0.0))
            .insert(Restitution::coefficient(1.0))
            .insert(Ccd::enabled());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("Barrier.png"),
            ..default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(490.,5.))
            .insert_bundle(TransformBundle{
                local: Transform{
                    translation: Vec3::new(-490.,-450.,0.),
                    rotation: Quat::from_rotation_z(-3.),
                    ..default()
                },
                global: default()
            })
            .insert(Friction::coefficient(0.0))
            .insert(Restitution::coefficient(1.0))
            .insert(Ccd::enabled());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("Barrier.png"),
            ..default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(500.,5.))
                    .insert_bundle(TransformBundle{
                local: Transform{
                    translation: Vec3::new(500.,-450.,0.),
                    rotation: Quat::from_rotation_z(3.),
                    ..default()
                },
                global: default()
            })
            .insert(Friction::coefficient(0.0))
            .insert(Restitution::coefficient(1.0))
            .insert(Ccd::enabled());
        
}