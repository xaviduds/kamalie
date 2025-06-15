use bevy::{color::palettes::css::PURPLE, core_pipeline::bloom::Bloom, prelude::*};
mod plugins;

const PLAYER_SPEED: f32 = 500.;
const CAMERA_DECAY_RATE: f32 = 10.;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(
            RunFixedMainLoop,
            (
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
                update_camera.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        )
        .run();
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct AccumulatedInput(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PhysicalTranslation(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1100., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));

    let size: f32 = 100.0;
    commands.spawn((
        Player,
        Name::new("Player"),
        Sprite::from_color(PURPLE, vec2(size, size)),
        Transform::from_scale(Vec3::splat(0.3)),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Bloom::NATURAL,
    ));
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut Velocity)>,
) {
    for (mut input, mut velocity) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            input.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            input.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            input.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            input.x -= 1.0;
        }

        velocity.0 = input.extend(0.0).normalize_or_zero() * PLAYER_SPEED;
    }
}

fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut AccumulatedInput,
        &mut Velocity,
    )>,
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        velocity,
    ) in query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

        input.0 = Vec2::ZERO;
    }
}

fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in
        query.iter_mut()
    {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        let alpha = fixed_time.overstep_fraction();

        let rendered_transform = previous.lerp(current, alpha);
        transform.translation = rendered_transform;
    }
}
