use bevy::prelude::*;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin{
    fn build(&self, app: &mut App){
        app
            .add_startup_system(display_score)
            .add_system(update_score)
    ;}
}

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Component)]
pub struct Score{
    pub player: i32,
    pub ai: i32
}

fn display_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: asset_server.load("space age.ttf"),
                        font_size: 72.,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("space age.ttf"),
                    font_size: 72.,
                    color: Color::WHITE,
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScoreBoard);

    commands
        .spawn()
        .insert(Score{
            player:0,
            ai:0
        });
}

fn update_score(
    mut query: Query<&mut Text, With<ScoreBoard>>,
    q_score: Query<&Score>
){
    let mut text = query.single_mut();
    let score = q_score.single();
    text.sections[0].value = format!("{}:{}", score.player, score.ai);
}