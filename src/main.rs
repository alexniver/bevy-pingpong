use bevy::{prelude::*, window::close_on_esc};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, close_on_esc)
        .run();
}
