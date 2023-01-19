use bevy::prelude::*;

pub struct FirstPlugin {}
impl Plugin for FirstPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(|| println!("FirstPlugin startup system"));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FirstPlugin {})
        .run();
}
