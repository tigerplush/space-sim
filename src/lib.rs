use bevy::prelude::*;
use camera::CameraPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(CameraPlugin::default());
    }
}
