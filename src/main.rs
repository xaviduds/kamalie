use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .with_hot_patch(|app: &mut App| {
            app.add_systems(StartupRerunHotPatch, setup);
        })
        .run();
}

const X_EXTENT: f32 = 900.;

#[hot(hot_patch_signature = true)]
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Hello world");

    commands.queue(|world: &mut World| {
        world.spawn(Camera2d);
        world.resource_scope(|world, mut asset_server: Mut<AssetServer>| {
            world.spawn(Sprite {
                image: asset_server.load("tech/rust.svg"),
                ..default()
            });
        });
    });
}
