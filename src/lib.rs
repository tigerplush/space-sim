use avian3d::prelude::*;
use bevy::{prelude::*, window::CursorOptions};
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            // primary_window: Window {
            //     cursor_options: CursorOptions {
            //         grab_mode: bevy::window::CursorGrabMode::Locked,
            //         visible: false,
            //         ..default()
            //     },
            //     ..default()
            // }
            // .into(),
            ..default()
        }))
        .add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::default(),
            PhysicsPlugins::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
            EnhancedInputPlugin,
        ))
        .add_plugins(player::plugin)
        .add_systems(Startup, setup);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/testlevel.glb"))),
        Collider::cuboid(10.0, 0.1, 10.0),
        RigidBody::Static,
    ));
}
