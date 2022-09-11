use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
pub struct BallPlugin;

impl Plugin for BallPlugin{
    fn build(&self, app:&mut App){
        app
            .add_startup_system(spawn_ball)
            .add_system(reset_ball)
            .add_system(ball_sound)
    ;}
}

#[derive(Component)]
pub struct Ball;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    
    commands
        .spawn_bundle(MaterialMesh2dBundle{
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(4.)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(AdditionalMassProperties::MassProperties(MassProperties{
            local_center_of_mass: Vec2::new(1.0,1.0),
            mass: 0.0001,
            principal_inertia: 1.
            }))
        .insert(Collider::ball(1.))
        .insert(Restitution::coefficient(1.))
        .insert(ColliderMassProperties::Density(2.0))
        .insert(GravityScale(0.0))
        .insert(ExternalImpulse {
            impulse: Vec2::new(5.0, 5.0),
            torque_impulse: 0.0,
        })
        .insert(Ccd::enabled())
        .insert(Velocity::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollidingEntities::default())
        .insert(Ball);
}

fn reset_ball(
    mut query: Query<(&mut ExternalImpulse, &mut Transform, &mut Velocity), With<Ball>>,
    mut q_score: Query<&mut crate::scoreboard::Score>,
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>
){
    let mut rng = thread_rng();
    let (mut ext_impulse, mut transform, mut velocity) = query.single_mut();
    let mut score = q_score.single_mut();

    let speed_multiplier: f32 = ((score.ai as f32) + (score.player as f32)+ 10.0 )/2.0;

    let direction: Vec2 = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
    if transform.translation.x > 1000.0 || transform.translation.x < -1000.0{
        if transform.translation.x > 1000.0 { score.player += 1; }
        else { score.ai += 1; }

        transform.translation = Vec3::new(0.,0.,0.);
        ext_impulse.impulse = direction * speed_multiplier;
        velocity.linvel = direction * speed_multiplier;
        velocity.angvel = 0.0;
        let music = asset_server.load("score.ogg");
        audio.play(music);
    }
    else if transform.translation.y > 550.0 || transform.translation.y < -550.0{
        transform.translation = Vec3::new(0.,0.,0.);
        ext_impulse.impulse = direction * speed_multiplier;
        velocity.linvel = direction * speed_multiplier;
        velocity.angvel = 0.0;
        let music = asset_server.load("score.ogg");
        audio.play(music);
    }
}

fn ball_sound(
    mut query: Query<(&mut Velocity, &CollidingEntities), With<Ball>>,
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>
) {
    for (mut velocity, collisions) in query.iter_mut() {
        for _ in collisions.iter() {
                let music = asset_server.load("ball_hit.ogg");
                audio.play(music);
                velocity.linvel *= 1.05;
        }
    
    }
}

