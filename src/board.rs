use bevy::{color::*, prelude::*, sprite::*};

#[derive(Component)]
pub struct Board {
    board: [[u8; 4]; 4],
}
fn translate(v: [i32; 2]) -> Vec2 {
    Vec2::new(v[0] as f32 * 100.0 - 150.0, v[1] as f32 * 100.0 - 150.0)
}

pub fn spawn_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for x in 0..4 {
        let circle_mesh = Mesh2dHandle(meshes.add(Circle { radius: 20.0 }));
        commands.spawn(MaterialMesh2dBundle {
            mesh: circle_mesh.clone(),
            material: materials.add(ColorMaterial::from_color(palettes::tailwind::GRAY_700)),
            transform: Transform::from_xyz(x as f32 * 100.0 - 150.0, -150.0, 10.0),
            ..default()
        });
        commands.spawn(MaterialMesh2dBundle {
            mesh: circle_mesh.clone(),
            material: materials.add(ColorMaterial::from_color(palettes::css::WHITE_SMOKE)),
            transform: Transform::from_xyz(x as f32 * 100.0 - 150.0, 150.0, 20.0),
            ..default()
        });
        let rec_mesh = Mesh2dHandle(meshes.add(Rectangle {
            half_size: Vec2::new(150.0, 150.0),
        }));
        commands.spawn(MaterialMesh2dBundle {
            mesh: rec_mesh.clone(),
            material: materials.add(ColorMaterial::from_color(palettes::css::SANDY_BROWN)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
    }
}
