use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "zenn-2022-06-04".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_stage_before(
            StartupStage::Startup,
            StartupPlayer,
            SystemStage::parallel(),
        )
        .add_startup_system_to_stage(StartupPlayer, setup_player)
        .add_startup_system(setup_ui)
        .add_startup_system(setup_lights)
        .add_startup_system(setup_objects)
        .add_system(update_player)
        .add_system(update_ui.after(update_player))
        .run();
}

#[derive(StageLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct StartupPlayer;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    transform_bundle: TransformBundle,
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(PlayerBundle {
            player: Player,
            transform_bundle: Transform::default().looking_at(Vec3::X, Vec3::Y).into(),
        })
        .with_children(|parent| {
            parent.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform {
                    translation: Vec3::Y,
                    ..default()
                },
                ..default()
            });
        });
}

fn update_player(
    mut player_transforms: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = match player_transforms.get_single_mut() {
        Ok(player_transform) => player_transform,
        _ => {
            error!("Transform not found.");
            return;
        }
    };

    let mut move_forward = 0.0f32;
    let mut turn_left = 0.0f32;

    const MOVE_UNIT: f32 = 10.0;
    const TURN_UNIT: f32 = 1.0;

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        move_forward += MOVE_UNIT;
    }

    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        move_forward -= MOVE_UNIT;
    }

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        turn_left += TURN_UNIT;
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        turn_left -= TURN_UNIT;
    }

    if move_forward != 0.0 {
        let translation = player_transform.forward() * (move_forward * time.delta_seconds());
        player_transform.translation += translation;
    }

    if turn_left != 0.0 {
        let rotation = Quat::from_rotation_y(turn_left * time.delta_seconds());
        player_transform.rotate(rotation);
    }
}

fn setup_lights(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle::default());
}

fn setup_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });
}

fn format_transform(player_transform: &Transform) -> String {
    let translation = player_transform.translation;

    format!(
        "X: {:.3}, Y: {:.3}, Z: {:.3}",
        translation.x, translation.y, translation.z
    )
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_transforms: Query<&Transform, With<Player>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let player_transform = match player_transforms.get_single() {
        Ok(player_transform) => player_transform,
        _ => {
            error!("Transform not found.");
            return;
        }
    };

    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            format_transform(player_transform),
            TextStyle {
                font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Left,
                ..default()
            },
        ),
        ..default()
    });
}

fn update_ui(
    mut texts: Query<&mut Text>,
    player_transforms: Query<&Transform, (With<Player>, Changed<Transform>)>,
) {
    let player_transform = match player_transforms.get_single() {
        Ok(player_transform) => player_transform,
        _ => {
            return;
        }
    };

    let mut text = match texts.get_single_mut() {
        Ok(text) => text,
        _ => {
            error!("Text not found.");
            return;
        }
    };

    let mut text_section = match text.sections.get_mut(0) {
        Some(text_section) => text_section,
        _ => {
            error!("TextSection not found.");
            return;
        }
    };

    text_section.value = format_transform(player_transform);
}
