use bevy::prelude::*;

pub fn app() {
    let window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "bevy_template".into(),
        ..default()
    };

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(window_descriptor);

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(debug);

    app.run();

}

fn debug() {

}
