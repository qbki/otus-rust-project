mod components;
mod consts;
mod events;
mod math;
mod resources;
mod systems;

use bevy::prelude::*;
use components::*;
use consts::*;
use events::*;
use resources::*;
use systems::*;

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut handlers: ResMut<Handlers>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    });

    let projectile_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.1, 0.1),
        ..default()
    });
    handlers
        .materials
        .insert(PROJECTILE_MATERIAL.to_string(), projectile_material_handle);

    let ship_mesh = asset_server.load("models/ship.glb#Mesh0/Primitive0");
    let enemy_mesh = asset_server.load("models/enemy.glb#Mesh0/Primitive0");
    let projectile_mesh = asset_server.load("models/projectile.glb#Mesh0/Primitive0");
    handlers
        .meshes
        .insert(PROJECTILE_MESH.to_string(), projectile_mesh);

    // ***** Player *****
    commands
        .spawn_bundle(PbrBundle {
            mesh: ship_mesh,
            material: material_handle.clone(),
            transform: Transform::from_xyz(-10.0, 0.0, 0.0),
            ..default()
        })
        .insert(Player::new())
        .insert(Weapons(vec![Weapon {
            speed: 20.0,
            muzzle_distance: 1.2,
            ..default()
        }]))
        .insert(Collider::new(1.0))
        .insert(Lives(3))
        .insert(Speed(10.0));

    // ***** Enemies *****
    {
        let enemy_weapon = Weapon {
            max_time: 0.5,
            is_shooting: true,
            speed: 4.0,
            muzzle_distance: 1.7,
            ..default()
        };
        let enemy_weapons = vec![
            Weapon {
                direction: Vec3::new(0.0, 1.0, 0.0),
                ..enemy_weapon
            },
            Weapon {
                direction: Vec3::new(0.0, -1.0, 0.0),
                ..enemy_weapon
            },
            Weapon {
                direction: Vec3::new(1.0, 0.0, 0.0),
                ..enemy_weapon
            },
            Weapon {
                direction: Vec3::new(-1.0, 0.0, 0.0),
                ..enemy_weapon
            },
        ];

        for coord in [Vec3::new(4.0, 5.0, 0.0), Vec3::new(4.0, -5.0, 0.0)] {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: enemy_mesh.clone(),
                    material: material_handle.clone(),
                    transform: Transform::from_translation(coord),
                    ..default()
                })
                .insert(Enemy::default())
                .insert(Weapons(enemy_weapons.clone()))
                .insert(Collider::new(1.4))
                .insert(Lives(10))
                .insert(RotationSpeed(0.5));
        }
    }

    // ***** Walls *****
    {
        let radius = 1.4;
        let uv_sphere_mesh = meshes.add(
            (shape::UVSphere {
                radius,
                ..default()
            })
            .into(),
        );
        let coord = Vec3::new(4.0, 0.0, 0.0);
        commands
            .spawn_bundle(PbrBundle {
                mesh: uv_sphere_mesh,
                material: material_handle,
                transform: Transform::from_translation(coord),
                ..default()
            })
            .insert(Wall)
            .insert(Collider::new(radius));
    }

    // ***** Text *****
    {
        let text_style = TextStyle {
            font: asset_server.load("fonts/fugaz-one/FugazOne-Regular.ttf"),
            font_size: 60.0,
            color: Color::WHITE,
        };

        let node_style = Style {
            display: Display::None,
            margin: UiRect::all(Val::Auto),
            align_self: AlignSelf::Center,
            ..default()
        };

        commands
            .spawn_bundle(
                TextBundle::from_sections([
                    TextSection::new(
                        "Fire - left mouse button\nMove left - A\nMove right - D\nMove up - W\nMove down - S\nExit game - Esc",
                        TextStyle {
                            font: asset_server.load("fonts/fugaz-one/FugazOne-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    )
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { left: Val::Px(20.0), top: Val::Px(10.0), ..default() },
                    ..default()
                })
            );

        commands
            .spawn_bundle(
                TextBundle::from_sections([TextSection::new(
                    "Press Fire Button",
                    text_style.clone(),
                )])
                .with_style(Style {
                    display: Display::Flex,
                    ..node_style
                }),
            )
            .insert(StartScreenText);

        commands
            .spawn_bundle(
                TextBundle::from_sections([TextSection::new("You Win!!!", text_style.clone())])
                    .with_style(node_style.clone()),
            )
            .insert(WinScreenText);

        commands
            .spawn_bundle(
                TextBundle::from_sections([TextSection::new("You Lose...", text_style)])
                    .with_style(node_style),
            )
            .insert(LoseScreenText);
    }

    // ***** Light *****
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 4.0, 4.0),
        ..default()
    });

    // ***** Camera *****
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Otus Project".to_string(),
            ..default()
        })
        .insert_resource(Control::new())
        .insert_resource(Handlers::new())
        .insert_resource(Elapsed::new())
        .insert_resource(GameState::new())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(elapsed_system)
        .add_system(keyboard_input_system)
        .add_system(mouse_input_system)
        .add_system(player_control_system)
        .add_system(enemy_system)
        .add_system(projectile_system)
        .add_system(spawn_projectiles_system)
        .add_system(projectile_hit_actor_system)
        .add_system(projectile_hit_wall_system)
        .add_system(game_state_system)
        .add_system(player_lose_system)
        .add_system(player_win_system)
        .add_event::<StartGameEvent>()
        .add_event::<LoseGameEvent>()
        .add_event::<WinGameEvent>()
        .run();
}
