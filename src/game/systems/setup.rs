use bevy::{
    camera::ScalingMode,
    color::palettes::tailwind::{SKY_50, SKY_400, SKY_500, SKY_800, SLATE_950},
    prelude::*,
    sprite::Anchor,
};

use crate::{
    app::state::AppState,
    config::constants::{BALL_SIZE, BALL_SPEED, BRICK_SIZE, CANVAS_SIZE, DEFAULT_PADDLE_SIZE},
    game::{
        components::{
            ball::{Ball, Velocity},
            brick::{Brick, BrickKind},
            paddle::Paddle,
            wall::{HalfSize, RespawnBallArea, Wall},
        },
        resources::Score,
    },
};

pub(in crate::game) fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CANVAS_SIZE.x + BRICK_SIZE.x,
                min_height: CANVAS_SIZE.y + BRICK_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Wall(Plane2d::new(Vec2::X)),
        Transform::from_xyz(-CANVAS_SIZE.x / 2.0, 0.0, 0.0),
    ));

    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_X)),
        Transform::from_xyz(CANVAS_SIZE.x / 2.0, 0.0, 0.0),
    ));

    commands.spawn((
        Wall(Plane2d::new(Vec2::Y)),
        Transform::from_xyz(0.0, -CANVAS_SIZE.y / 2.0, 0.0),
    ));

    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_Y)),
        Transform::from_xyz(0.0, CANVAS_SIZE.y / 2.0, 0.0),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(CANVAS_SIZE + Vec2::splat(4.0)),
            color: Color::from(SKY_50),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -3.0),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(CANVAS_SIZE),
            color: Color::from(SKY_800),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -2.0),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(
                CANVAS_SIZE.x,
                CANVAS_SIZE.y / 8.0 - DEFAULT_PADDLE_SIZE.y / 2.0,
            )),
            color: Color::from(SKY_500).with_alpha(0.4),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, -CANVAS_SIZE.y / 2.0, -1.0),
        RespawnBallArea,
    ));
}

pub(in crate::game) fn new_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    score.points = 0;

    commands.spawn((
        Sprite {
            custom_size: Some(DEFAULT_PADDLE_SIZE),
            color: SKY_50.into(),
            ..default()
        },
        Transform::from_xyz(0.0, -CANVAS_SIZE.y * (3.0 / 8.0), 0.0),
        Paddle,
        DespawnOnExit(AppState::Playing),
        HalfSize(DEFAULT_PADDLE_SIZE / 2.0),
    ));

    commands.spawn((
        Ball,
        Velocity(Vec2::new(-200.0, -400.0).normalize() * BALL_SPEED),
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::from(SLATE_950))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        DespawnOnExit(AppState::Playing),
        children![(
            Mesh2d(meshes.add(Circle::new(BALL_SIZE - 1.0))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, 0.0, 1.0),
        )],
    ));

    spawn_bricks(&mut commands, &mut meshes, &mut materials, &asset_server);
}

pub(in crate::game) fn spawn_bricks(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    asset_server: &AssetServer,
) {
    let bricks_per_row = 13;
    let rows = 6;
    let base_color = Oklcha::from(SKY_400);
    let tnt_icon: Handle<Image> = asset_server.load("icons/tnt.png");
    let extra_ball_icon: Handle<Image> = asset_server.load("icons/extra_ball.png");
    let clear_row_icon: Handle<Image> = asset_server.load("icons/clear_row.png");
    let clear_column_icon: Handle<Image> = asset_server.load("icons/clear_column.png");
    let rainbow_gradient: Handle<Image> = asset_server.load("textures/rainbow_gradient.png");

    for row in 0..rows {
        for column in 0..bricks_per_row {
            let kind = brick_kind_for_position(row, column);
            let hue = ((row + column) % 8) as f32 * (360.0 / 8.0);
            let normal_color = Color::from(base_color.with_hue(hue));
            let color = match kind {
                BrickKind::Normal => normal_color,
                BrickKind::Tnt => Color::srgb(0.96, 0.18, 0.12),
                BrickKind::ExtraBall => Color::srgb(0.14, 0.90, 0.98),
                BrickKind::ClearRow | BrickKind::ClearColumn => Color::WHITE,
            };

            let x = BRICK_SIZE.x * column as f32 - BRICK_SIZE.x * bricks_per_row as f32 / 2.0
                + BRICK_SIZE.x / 2.0;
            let y = CANVAS_SIZE.y * (3.0 / 8.0) - BRICK_SIZE.y * row as f32;
            let is_gradient = matches!(kind, BrickKind::ClearRow | BrickKind::ClearColumn);
            let icon = match kind {
                BrickKind::Normal => None,
                BrickKind::Tnt => Some(tnt_icon.clone()),
                BrickKind::ExtraBall => Some(extra_ball_icon.clone()),
                BrickKind::ClearRow => Some(clear_row_icon.clone()),
                BrickKind::ClearColumn => Some(clear_column_icon.clone()),
            };

            let brick_entity = commands
                .spawn((
                    Brick {
                        row,
                        column,
                        kind,
                        color,
                    },
                    Sprite {
                        custom_size: Some(BRICK_SIZE),
                        color: if is_gradient {
                            Color::from(SKY_50)
                        } else {
                            color.with_alpha(0.4)
                        },
                        ..default()
                    },
                    Transform::from_xyz(x, y, 0.0),
                    HalfSize(BRICK_SIZE / 2.0),
                    DespawnOnExit(AppState::Playing),
                ))
                .id();

            commands.entity(brick_entity).with_children(|parent| {
                if is_gradient {
                    parent.spawn((
                        Sprite {
                            image: rainbow_gradient.clone(),
                            custom_size: Some(BRICK_SIZE - Vec2::splat(4.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 1.0),
                    ));
                } else {
                    parent.spawn((
                        Mesh2d(meshes.add(Rectangle::new(BRICK_SIZE.x - 2.0, BRICK_SIZE.y - 2.0))),
                        MeshMaterial2d(materials.add(color)),
                        Transform::from_xyz(0.0, 0.0, 1.0),
                    ));
                }

                if let Some(icon) = icon {
                    parent.spawn((
                        Sprite {
                            image: icon,
                            custom_size: Some(Vec2::new(32.0, 21.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 2.0),
                    ));
                }
            });
        }
    }
}

fn brick_kind_for_position(row: usize, column: usize) -> BrickKind {
    match (row, column) {
        (1, 3) => BrickKind::Tnt,
        (2, 6) => BrickKind::ExtraBall,
        (3, 1) => BrickKind::ClearRow,
        (4, 10) => BrickKind::ClearColumn,
        _ => BrickKind::Normal,
    }
}
