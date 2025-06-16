use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind, pbr::NotShadowCaster, prelude::*, render::view::RenderLayers,
};
use bevy_enhanced_input::{
    input::Input,
    prelude::*,
    preset::{Axial, Cardinal},
};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

pub fn plugin(app: &mut App) {
    app.register_type::<IntendedMovement>()
        .add_input_context::<DefaultInputContext>()
        .add_systems(Startup, setup)
        .add_systems(Update, sync_camera)
        .add_systems(
            FixedUpdate,
            apply_movement.in_set(TnuaUserControlsSystemSet),
        )
        .add_observer(setup_player)
        .add_observer(default_binding)
        .add_observer(rotate_camera_yaw_and_pitch)
        .add_observer(record_movement)
        .add_observer(reset_movement);
}

#[derive(Component)]
struct Player;

pub(crate) const PLAYER_RADIUS: f32 = 0.5;
const PLAYER_CAPSULE_LENGTH: f32 = 1.0;
const PLAYER_HEIGHT: f32 = PLAYER_CAPSULE_LENGTH + 2.0 * PLAYER_RADIUS;
const PLAYER_HALF_HEIGHT: f32 = PLAYER_HEIGHT / 2.0;
const PLAYER_FLOAT_HEIGHT: f32 = PLAYER_HALF_HEIGHT + 0.01;
const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));
    commands
        .spawn_empty()
        .insert((
            Player,
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::Inherited,
            Name::new("Player"),
        ))
        .with_children(|parent| {
            // Spawn view model camera.
            parent.spawn((
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            // Spawn the player's right arm.
            parent.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ));
        });

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        PlayerCamera,
    ));
}

fn setup_player(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.entity(trigger.target()).insert((
        RigidBody::Dynamic,
        Collider::capsule(PLAYER_RADIUS, PLAYER_CAPSULE_LENGTH),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_RADIUS - 0.01, 0.0)),
        LockedAxes::ROTATION_LOCKED,
        Friction::ZERO,
        ColliderDensity(100.0),
        Actions::<DefaultInputContext>::default(),
    ));
}

#[derive(Debug, InputAction)]
#[input_action(output = Vec3)]
struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Rotate;

#[derive(InputContext)]
struct DefaultInputContext;

fn default_binding(
    _trigger: Trigger<Binding<DefaultInputContext>>,
    player: Single<&mut Actions<DefaultInputContext>>,
) {
    let mut actions = player.into_inner();

    const DEFAULT_SPEED: f32 = 8.0;
    actions
        .bind::<Move>()
        .to((Cardinal::wasd_keys(), Axial::left_stick()))
        .with_modifiers((
            DeadZone::default(),
            SmoothNudge::default(),
            Negate::y(),
            Scale::splat(DEFAULT_SPEED),
            SwizzleAxis::XZY,
        ));

    const DEFAULT_SENSITIVITY: f32 = 0.002;
    actions
        .bind::<Rotate>()
        .to((Input::mouse_motion(), Axial::right_stick()))
        .with_modifiers((Negate::all(), Scale::splat(DEFAULT_SENSITIVITY)));
}

#[derive(Component)]
struct PlayerCamera;

fn rotate_camera_yaw_and_pitch(
    trigger: Trigger<Fired<Rotate>>,
    mut transform: Single<&mut Transform, With<PlayerCamera>>,
) {
    let delta = trigger.value;
    if delta == Vec2::ZERO {
        return;
    }

    let delta_yaw = delta.x;
    let delta_pitch = delta.y;

    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;

    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct IntendedMovement(Vec3);

fn record_movement(trigger: Trigger<Fired<Move>>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(IntendedMovement(trigger.value));
}

fn reset_movement(trigger: Trigger<Completed<Move>>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(IntendedMovement(Vec3::ZERO));
}

fn apply_movement(
    player_controller: Single<(&mut TnuaController, &IntendedMovement)>,
    transform: Single<&Transform, With<PlayerCamera>>,
) {
    let (mut controller, movement) = player_controller.into_inner();
    let (yaw, _pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw);
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: yaw_quat * movement.0,
        float_height: PLAYER_FLOAT_HEIGHT,
        ..default()
    });
}

fn sync_camera(
    player: Single<&Transform, (With<Player>, Without<PlayerCamera>)>,
    mut player_camera_parent: Single<&mut Transform, With<PlayerCamera>>,
) {
    const CAMERA_HEIGHT: f32 = 1.7;
    player_camera_parent.translation =
        player.translation + Vec3::Y * (CAMERA_HEIGHT - PLAYER_FLOAT_HEIGHT);
}
