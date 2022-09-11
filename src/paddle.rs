use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin{
    fn build(&self, app: &mut App){
        app
            .add_startup_system(spawn_paddles)
            .add_system(paddle_controller)
            .add_system(paddle_ai)
    ;}
}

#[derive(Component)]
pub struct RPaddle;

#[derive(Component)]
pub struct LPaddle;

fn spawn_paddles(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("Paddle-Right.png"),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(750.,0.,0.)))
        .with_children(|children|{
            children
                .spawn()
                    .insert(RigidBody::Fixed)
                    .insert(Collider::cuboid(1.,15.))
                    .insert(Friction::coefficient(0.0))
                    .insert(Restitution::coefficient(1.0))
                    .insert(Ccd::enabled())
                    .insert_bundle(TransformBundle{
                        local: Transform{
                            translation: Vec3::new(0.,15.,0.),
                            rotation: Quat::from_rotation_z(-176.),
                            ..default()
                        },
                        global: default()
                    });

            children
                .spawn()
                    .insert(RigidBody::Fixed)
                    .insert(Collider::cuboid(1.,15.))
                    .insert(Friction::coefficient(0.0))
                    .insert(Restitution::coefficient(1.0))
                    .insert(Ccd::enabled())
                    .insert_bundle(TransformBundle{
                        local: Transform{
                            translation: Vec3::new(0.,-15.,0.),
                            rotation: Quat::from_rotation_z(176.),
                            ..default()
                        },
                        global: default()
                    });
            })
        .insert(RPaddle);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("Paddle-Left.png"),
            ..default()
        })
            .insert_bundle(TransformBundle::from(Transform::from_xyz(-750.,0.,0.)))
            .with_children(|children|{

                children
                    .spawn()
                        .insert(RigidBody::Fixed)
                        .insert(Collider::cuboid(1.,15.))
                        .insert(Friction::coefficient(0.0))
                        .insert(Restitution::coefficient(1.0))
                        .insert(Ccd::enabled())
                        .insert_bundle(TransformBundle{
                            local: Transform{
                                translation: Vec3::new(0.,15.,0.),
                                rotation: Quat::from_rotation_z(176.),
                                ..default()
                            },
                            global: default()
                        });

                children
                    .spawn()
                        .insert(RigidBody::Fixed)
                        .insert(Collider::cuboid(1.,15.))
                        .insert(Friction::coefficient(0.0))
                        .insert(Restitution::coefficient(1.0))
                        .insert(Ccd::enabled())
                        .insert_bundle(TransformBundle{
                            local: Transform{
                                translation: Vec3::new(0.,-15.,0.),
                                rotation: Quat::from_rotation_z(-176.),
                                ..default()
                            },
                            global: default()
                        });
                })
            .insert(LPaddle);
}

fn paddle_controller(
    mut query: Query<&mut Transform, With<LPaddle>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    let mut t_paddle = query.single_mut();

    if input.pressed(KeyCode::W){
        t_paddle.translation += Vec3::new(0.0,500.0,0.0) * time.delta_seconds();
    }
    else if input.pressed(KeyCode::S){
        t_paddle.translation += Vec3::new(0.0,-500.0,0.0) * time.delta_seconds();
    }

    if t_paddle.translation.y > 500.0{
        t_paddle.translation.y = 500.0;
    }
    else if t_paddle.translation.y < -500.0{
        t_paddle.translation.y = -500.0;
    }
}

fn paddle_ai(
    mut query: Query<&mut Transform, (With<RPaddle>, Without<crate::ball::Ball>)>,
    q_ball: Query<&Transform, (With<crate::ball::Ball>, Without<RPaddle>)>,
    time: Res<Time>,
){
    let mut t_paddle = query.single_mut();
    let t_ball = q_ball.single();

    let tolerance:f32 = 20.0;

    let difference = t_ball.translation.y - t_paddle.translation.y;

    if difference > tolerance || difference < tolerance * -1.0 {
        if difference > 0.0{
            t_paddle.translation += Vec3::new(0.0,500.,0.0) * time.delta_seconds();
        }
        else {
            t_paddle.translation += Vec3::new(0.0,-500.,0.0) * time.delta_seconds();
        }
    }

    if t_paddle.translation.y > 500.0{
        t_paddle.translation.y = 500.0;
    }
    else if t_paddle.translation.y < -500.0{
        t_paddle.translation.y = -500.0;
    }
}