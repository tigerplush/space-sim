use bevy::prelude::*;

#[derive(Clone, Copy, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct CameraSettings {}

#[derive(Default)]
pub struct CameraPlugin(CameraSettings);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0).add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
