use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::math::Vec4Swizzles;

const SPEED: f32 = 0.2;

#[derive(Component)]
struct Player {
    plane: Plane,
}

impl Player {
    fn new() -> Self {
        Self {
            plane: Plane {
                origin: Vec3::ZERO,
                normal: Vec3::new(0.0, 0.0, 1.0),
            }
        }
    }
}

struct Ray {
    origin: Vec3,
    normal: Vec3,
}

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
}

impl Plane {
    fn hit_test(&self, ray: &Ray) -> Option<Vec3> {
        let dot_product = self.normal.dot(ray.normal);
        if dot_product.abs() > f32::EPSILON {
            let t = (self.origin - ray.origin).dot(self.normal) / dot_product;
            if t > 0.0 {
                Some(ray.origin + ray.normal * t)
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct Control {
    cursor_ray: Ray,
}

impl Control {
    fn new() -> Self {
        Control {
            cursor_ray: Ray {
                origin: Vec3::ZERO,
                normal: Vec3::new(0.0, 0.0, -1.0),
            }
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
) {
    let mut direction = Vec3::ZERO;
    let (_, mut player_transform) = query.single_mut();
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    player_transform.translation += direction.normalize_or_zero() * SPEED;
}

fn mouse_input_system(
    mut control: ResMut<Control>,
    mut mouse_motion_events: EventReader<CursorMoved>,
    mut camera: Query<(&Camera, &GlobalTransform, &Transform)>,
) {
    let (camera, camera_global_transform, &camera_transform) = camera.single_mut();
    if let Some(cursor_movement) = mouse_motion_events.iter().last() {
        let usign_screen_size = camera.physical_viewport_size().unwrap();
        let screen_size = Vec2::new(usign_screen_size.x as f32, usign_screen_size.y as f32);
        let screen_ndc_2d = (cursor_movement.position / screen_size) * 2.0 - Vec2::ONE;

        let ndc_to_world_matrix = camera_global_transform.compute_matrix() * camera.projection_matrix().inverse();

        control.cursor_ray.origin = camera_transform.translation;
        control.cursor_ray.normal = (camera_transform.translation - ndc_to_world_matrix.project_point3(screen_ndc_2d.extend(-1.0))).normalize();
    }
}

fn player_rotation_system(
    mut player: Query<(&Player, &mut Transform)>,
    control: Res<Control>,
) {
    let (player, mut player_transform) = player.single_mut();


    if let Some(hit_point) = player.plane.hit_test(&control.cursor_ray) {
        let angle_vector = hit_point - player_transform.translation;
        let angle = angle_vector.y.atan2(angle_vector.x);
        if angle.is_normal() {
            let mut matrix = Transform::from_translation(player_transform.translation);
            matrix.rotate_z(angle - PI * 0.5);
            *player_transform = matrix;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    });

    let ship_mesh = asset_server.load("ship.glb#Mesh0/Primitive0");

    commands
        .spawn_bundle(PbrBundle {
            mesh: ship_mesh,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Player::new());

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 4.0, 4.0),
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .add_system(mouse_input_system)
        .add_system(player_rotation_system)
        .insert_resource(Control::new())
        .run();
}
