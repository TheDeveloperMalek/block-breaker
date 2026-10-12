use bevy::{
    color::palettes::tailwind::SLATE_50, input::common_conditions::input_just_pressed, prelude::*,
};

use crate::{app::state::AppState, game::resources::Score};

const SCOREBOARD_BASE_FONT_SIZE: f32 = 32.0;
const SCOREBOARD_PULSE_EXTRA_SIZE: f32 = 6.0;
const SCOREBOARD_ANIMATION_SECONDS: f32 = 0.42;

#[derive(Component)]
struct ScoreboardText;

#[derive(Component)]
struct BestScoreText;

#[derive(Component)]
struct ScorePulse {
    last_score: u32,
    elapsed: f32,
    active: bool,
}

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), show_scoreboard)
            .add_systems(OnEnter(AppState::GameOver), show_restart_button)
            .add_systems(
                Update,
                (
                    update_scoreboard.run_if(in_state(AppState::Playing)),
                    restart_game
                        .run_if(in_state(AppState::GameOver))
                        .run_if(input_just_pressed(KeyCode::KeyR)),
                ),
            );
    }
}

fn show_restart_button(mut commands: Commands) {
    commands.spawn((
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
        DespawnOnExit(AppState::GameOver),
        children![(
            Text::new("Press R to Restart Game"),
            TextFont::from_font_size(48.0),
            TextColor(SLATE_50.into()),
        )],
    ));
}

fn restart_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}

fn show_scoreboard(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(0.0),
                top: px(12.0),
                width: percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
            DespawnOnExit(AppState::Playing),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("SCORE {:06}", score.points)),
                TextFont::from_font_size(SCOREBOARD_BASE_FONT_SIZE),
                TextColor(SLATE_50.into()),
                ScoreboardText,
                ScorePulse {
                    last_score: score.points,
                    elapsed: 0.0,
                    active: false,
                },
            ));
            parent.spawn((
                Text::new(format!("BEST {:06}", score.best_points)),
                TextFont::from_font_size(16.0),
                TextColor(Color::srgb(1.0, 0.82, 0.32)),
                BestScoreText,
            ));
        });
}

fn update_scoreboard(
    score: Res<Score>,
    time: Res<Time>,
    mut texts: Query<(
        &mut Text,
        Option<&mut TextFont>,
        Option<&mut ScorePulse>,
        Option<&ScoreboardText>,
        Option<&BestScoreText>,
    )>,
) {
    for (mut text, font, pulse, scoreboard, best_score) in &mut texts {
        if scoreboard.is_some() {
            let new_text = format!("SCORE {:06}", score.points);
            if text.0 != new_text {
                text.0 = new_text;
            }

            if let (Some(mut font), Some(mut pulse)) = (font, pulse) {
                if pulse.last_score != score.points {
                    pulse.last_score = score.points;
                    pulse.elapsed = 0.0;
                    pulse.active = true;
                }

                if pulse.active {
                    pulse.elapsed =
                        (pulse.elapsed + time.delta_secs()).min(SCOREBOARD_ANIMATION_SECONDS);
                    let progress = pulse.elapsed / SCOREBOARD_ANIMATION_SECONDS;
                    let intensity = (std::f32::consts::PI * progress).sin();
                    font.font_size = FontSize::Px(
                        SCOREBOARD_BASE_FONT_SIZE + SCOREBOARD_PULSE_EXTRA_SIZE * intensity,
                    );

                    if progress >= 1.0 {
                        font.font_size = FontSize::Px(SCOREBOARD_BASE_FONT_SIZE);
                        pulse.active = false;
                    }
                }
            }
        }

        if best_score.is_some() {
            let new_text = format!("BEST {:06}", score.best_points);
            if text.0 != new_text {
                text.0 = new_text;
            }
        }
    }
}
