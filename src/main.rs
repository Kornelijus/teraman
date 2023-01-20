use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct FirstPlugin {}
impl Plugin for FirstPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(|| println!("FirstPlugin startup system"));
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    #[cfg(debug_assertions)]
    {
        app.add_plugin(WorldInspectorPlugin);
    }

    app.add_plugin(FirstPlugin {}).run();
}
