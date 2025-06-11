use bevy::prelude::*;
use space_sim::AppPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(AppPlugin)
        .run()
}
