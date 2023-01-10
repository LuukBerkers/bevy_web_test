use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(bevy_web_test::setup)
        .add_system(bevy_web_test::animate)
        .run();
}
