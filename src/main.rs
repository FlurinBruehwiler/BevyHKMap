use std::fs;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "bevy_test_1".to_string(),
            resizable: false,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())

        .register_type::<Image>()
        .register_type::<ControlableCamera>()

        .add_startup_system(setup)

        .add_system(print_mouse_events_system)

        .run();
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Image;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ControlableCamera{
    start_pos: Vec2
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default()).insert(ControlableCamera{
        start_pos: Vec2::new(0.0, 0.0)
    });

    let folder = "C:/Programming/Github/bevy_test_1/assets/images";
    let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        let x = path.unwrap().path();
        let str_path = x.to_str().unwrap();

        info!(str_path);

        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load(str_path),
            ..default()
        })
            .insert(Image);
    }
}

fn print_mouse_events_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut ControlableCamera, &mut Transform)>,
    mut windows: ResMut<Windows>,
) {
    let (mut controlableCamera, mut transform) = query.single_mut();
    let window = windows.primary_mut();
    let pos = window.cursor_position();

    let wrld_pos = window_to_world(event.position, window, &transform);
    info!("{:?}", wrld_pos);

    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {

        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

fn window_to_world(
    position: Vec2,
    window: &Window,
    camera: &Transform,
) -> Vec3 {

    // Center in screen space
    let norm = Vec3::new(
        position.x - window.width() / 2.,
        position.y - window.height() / 2.,
        0.,
    );

    // Apply camera transform
    *camera * norm

    // Alternatively:
    //camera.mul_vec3(norm)
}
