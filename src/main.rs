use std::fs;
use std::path::Path;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
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
            resizable: true,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())

        .register_type::<Image>()
        .register_type::<ControlableCamera>()

        .add_startup_system(setup)

        .add_system(my_cursor_system)

        .run();
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Image;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ControlableCamera{
    mouse_start_pos: Vec2,
    camera_start_pos: Vec2
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default()).insert(ControlableCamera{
        mouse_start_pos: Vec2::new(0.0, 0.0),
        camera_start_pos: Vec2::new(0.0, 0.0)
    });

    let folder = "C:/Programming/Github/bevy_test_1/assets/images";
    let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        let path_path = path.unwrap().path();
        let str_path = path_path.to_str().unwrap();

        let ancestors = Path::new(&str_path).file_name().unwrap().to_str().unwrap();
        let slice = ancestors.replace(".jpg", "");

        let split = slice.split("_");
        let vec: Vec<&str> = split.collect();
        let x = vec.get(0).unwrap().to_string().parse::<f32>().unwrap();
        let y = vec.get(1).unwrap().to_string().parse::<f32>().unwrap();

        info!(ancestors);

        let bundle = SpriteBundle {
            texture: asset_server.load(str_path),
            transform: Transform::from_xyz(x * 1024.0, y * -1024.0, 0.0),
            ..default()
        };

        commands.spawn_bundle(bundle)
            .insert(Image);
    }
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    mut q_camera: Query<(&Camera, &mut Transform, &mut ControlableCamera)>,

    mouse_button_input: Res<Input<MouseButton>>,

    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, mut camera_transform , mut controllable_camera) = q_camera.single_mut();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        //eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);

        if mouse_button_input.just_pressed(MouseButton::Left) {
            controllable_camera.mouse_start_pos = world_pos;
            controllable_camera.camera_start_pos = Vec2::new(camera_transform.translation.x, camera_transform.translation.y);
        }

        if mouse_button_input.pressed(MouseButton::Left) {
            let vec_mouse = world_pos - controllable_camera.mouse_start_pos;
            // println!("{}/{}", vec_mouse.x, vec_mouse.y);
            let new_camera_translation = controllable_camera.camera_start_pos - vec_mouse;
            camera_transform.translation = Vec3::new(new_camera_translation.x, new_camera_translation.y, 0.0);
        }

        for event in mouse_wheel_events.iter() {
            let new_scale : Vec3 = camera_transform.scale - event.y;
            if new_scale.x > 0.0 && new_scale.y > 0.0 {
                camera_transform.scale = new_scale;
            }
            // info!("{:?}", event);
        }
    }
}