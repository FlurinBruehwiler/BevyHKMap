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

        .register_type::<Tower>()
        .register_type::<Lifetime>()

        .add_startup_system(asset_loading)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)

        .add_system(tower_shooting)
        .add_system(bullet_despawn)

        .run();
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>){
    commands.insert_resource(GameAssets{
        bullet_scene: assets.load("monkey.glb#Scene0"),
    })
}

pub struct GameAssets{
    bullet_scene: Handle<Scene>
}

fn tower_shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            commands.spawn_bundle(SceneBundle {
                scene: bullet_assets.bullet_scene.clone(),
                transform: Transform::from_xyz(0.0, 0.7, 0.6)
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..default()
            })
                .insert(Lifetime{
                    timer: Timer::from_seconds(0.5, false)
                })
                .insert(Name::new("Bullet"));
        }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>
){
    for (entity, mut bullet) in &mut bullets{
        bullet.timer.tick(time.delta());
        if bullet.timer.just_finished(){
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }).insert(Name::new("Ground"));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, true)
        })
        .insert(Name::new("Tower"));

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
        .insert(Name::new("Light"));
}