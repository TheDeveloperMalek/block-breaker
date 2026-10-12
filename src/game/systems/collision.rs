use std::{
    collections::HashSet,
    f32::consts::{FRAC_PI_4, PI},
};

use bevy::{color::palettes::tailwind::SLATE_950, prelude::*};

use crate::{
    app::state::AppState,
    audio::plugin::SoundEffect,
    config::constants::{BALL_SIZE, BALL_SPEED, CANVAS_SIZE},
    game::{
        components::{
            ball::{Ball, Velocity},
            brick::{Brick, BrickKind},
            paddle::Paddle,
            wall::{HalfSize, Wall},
        },
        resources::Score,
    },
};

#[derive(Clone, Copy)]
struct BrickContact {
    entity: Entity,
    normal: Vec2,
    center: Vec2,
    brick: Brick,
}

pub(super) fn circle_aabb_normal(
    circle_center: Vec2,
    radius: f32,
    box_center: Vec2,
    half_size: Vec2,
) -> Option<Vec2> {
    let min = box_center - half_size;
    let max = box_center + half_size;
    let closest = circle_center.clamp(min, max);
    let difference = circle_center - closest;
    let distance_squared = difference.length_squared();

    if distance_squared > radius * radius {
        return None;
    }

    if distance_squared > f32::EPSILON {
        return Some(difference.normalize());
    }

    let distances = [
        (circle_center.x - min.x, Vec2::NEG_X),
        (max.x - circle_center.x, Vec2::X),
        (circle_center.y - min.y, Vec2::NEG_Y),
        (max.y - circle_center.y, Vec2::Y),
    ];

    distances
        .into_iter()
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, normal)| normal)
}

pub(in crate::game) fn ball_movement(
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    colliders: Query<
        (
            Entity,
            &Transform,
            &HalfSize,
            Option<&Brick>,
            Option<&Paddle>,
        ),
        Without<Ball>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    mut sound_effects: MessageWriter<SoundEffect>,
    mut score: ResMut<Score>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut broken_bricks = HashSet::new();

    for (mut transform, mut velocity) in &mut balls {
        let dt = time.delta_secs();
        let position = transform.translation.truncate();
        let next_position = position + velocity.0 * dt;
        let mut wall_hit = false;

        for (wall, wall_transform) in &walls {
            let normal = wall.0.normal.as_vec2();
            let distance = (next_position - wall_transform.translation.truncate()).dot(normal);

            if distance < BALL_SIZE && velocity.0.dot(normal) < 0.0 {
                velocity.0 = velocity.0.reflect(normal);
                wall_hit = true;
            }
        }

        let mut new_position = position + velocity.0 * dt;
        let half_canvas = CANVAS_SIZE / 2.0;

        if new_position.x < -half_canvas.x + BALL_SIZE {
            new_position.x = -half_canvas.x + BALL_SIZE;
            velocity.0.x = velocity.0.x.abs();
            wall_hit = true;
        } else if new_position.x > half_canvas.x - BALL_SIZE {
            new_position.x = half_canvas.x - BALL_SIZE;
            velocity.0.x = -velocity.0.x.abs();
            wall_hit = true;
        }

        if new_position.y > half_canvas.y - BALL_SIZE {
            new_position.y = half_canvas.y - BALL_SIZE;
            velocity.0.y = -velocity.0.y.abs();
            wall_hit = true;
        }

        if wall_hit {
            sound_effects.write(SoundEffect::WallHit);
        }

        let mut paddle_collision = None;
        let mut brick_collisions = Vec::new();

        for (entity, collider_transform, half_size, brick, paddle) in &colliders {
            let center = collider_transform.translation.truncate();
            let Some(normal) = circle_aabb_normal(new_position, BALL_SIZE, center, half_size.0)
            else {
                continue;
            };

            if velocity.0.dot(normal) >= 0.0 {
                continue;
            }

            if paddle.is_some() {
                paddle_collision = Some((center, normal, half_size.0));
            } else if let Some(brick) = brick {
                if !broken_bricks.contains(&entity) {
                    brick_collisions.push(BrickContact {
                        entity,
                        normal,
                        center,
                        brick: *brick,
                    });
                }
            }
        }

        if let Some((center, normal, half_size)) = paddle_collision {
            sound_effects.write(SoundEffect::PaddleHit);
            let hit_offset = (new_position.x - center.x) / half_size.x;
            let angle = FRAC_PI_4.lerp(PI - FRAC_PI_4, (hit_offset + 1.0) / 2.0);
            velocity.0 = Vec2::from_angle(angle).normalize() * BALL_SPEED;
            velocity.0.y = velocity.0.y.abs();
            new_position += normal * 1.0;
        } else if !brick_collisions.is_empty() {
            let reflection_normal = brick_collisions[0].normal;
            let mut pending = brick_collisions;
            let mut destroyed = Vec::new();

            while let Some(contact) = pending.pop() {
                if !broken_bricks.insert(contact.entity) {
                    continue;
                }

                destroyed.push(contact.entity);

                match contact.brick.kind {
                    BrickKind::Normal => {}
                    BrickKind::ExtraBall => {
                        spawn_bonus_ball(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            contact.center,
                            contact.brick.color,
                            velocity.0,
                        );
                    }
                    BrickKind::Tnt => {
                        for (other_entity, other_transform, _, other_brick, _) in &colliders {
                            let Some(other_brick) = other_brick else {
                                continue;
                            };

                            let is_neighbor = contact.brick.row.abs_diff(other_brick.row) <= 1
                                && contact.brick.column.abs_diff(other_brick.column) <= 1;

                            if is_neighbor && !broken_bricks.contains(&other_entity) {
                                pending.push(BrickContact {
                                    entity: other_entity,
                                    normal: contact.normal,
                                    center: other_transform.translation.truncate(),
                                    brick: *other_brick,
                                });
                            }
                        }
                    }
                    BrickKind::ClearRow => {
                        for (other_entity, other_transform, _, other_brick, _) in &colliders {
                            let Some(other_brick) = other_brick else {
                                continue;
                            };

                            if contact.brick.row == other_brick.row
                                && !broken_bricks.contains(&other_entity)
                            {
                                pending.push(BrickContact {
                                    entity: other_entity,
                                    normal: contact.normal,
                                    center: other_transform.translation.truncate(),
                                    brick: *other_brick,
                                });
                            }
                        }
                    }
                    BrickKind::ClearColumn => {
                        for (other_entity, other_transform, _, other_brick, _) in &colliders {
                            let Some(other_brick) = other_brick else {
                                continue;
                            };

                            if contact.brick.column == other_brick.column
                                && !broken_bricks.contains(&other_entity)
                            {
                                pending.push(BrickContact {
                                    entity: other_entity,
                                    normal: contact.normal,
                                    center: other_transform.translation.truncate(),
                                    brick: *other_brick,
                                });
                            }
                        }
                    }
                }
            }

            for entity in &destroyed {
                commands.entity(*entity).despawn();
            }

            let points = match destroyed.len() {
                1 => 15,
                2 => 50,
                _ => 100,
            };
            score.points += points;
            score.best_points = score.best_points.max(score.points);
            sound_effects.write(SoundEffect::BrickHit);
            velocity.0 = velocity.0.reflect(reflection_normal);
            new_position += reflection_normal * 1.0;
        }

        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
    }
}

fn spawn_bonus_ball(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    color: Color,
    source_velocity: Vec2,
) {
    let mut direction = Vec2::new(-source_velocity.x, source_velocity.y);
    if direction.length_squared() <= f32::EPSILON {
        direction = Vec2::new(0.35, -1.0);
    }
    direction = direction.normalize();

    commands
        .spawn((
            Ball,
            Velocity(direction * BALL_SPEED),
            Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
            MeshMaterial2d(materials.add(Color::from(SLATE_950))),
            Transform::from_xyz(position.x, position.y, 0.0),
            DespawnOnExit(AppState::Playing),
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(BALL_SIZE - 1.0))),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(0.0, 0.0, 1.0),
            ));
        });
}
