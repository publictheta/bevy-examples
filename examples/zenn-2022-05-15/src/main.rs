use bevy::{prelude::*, render::camera::Camera3d};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "zenn-2022-05-15".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<Player>()
        .add_event::<PlayerMoveEvent>()
        .add_startup_system(setup_lights)
        .add_startup_system(setup_cameras)
        .add_startup_system(setup_objects)
        .add_startup_system(setup_texts)
        .add_system(update)
        .add_system(update_cameras.after(update))
        .add_system(update_texts.after(update))
        .run();
}

struct Player {
    transform: Transform,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            transform: Transform::default().looking_at(Vec3::X, Vec3::Y),
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let translation = self.transform.translation;

        write!(
            f,
            "X: {:.3}, Y: {:.3}, Z: {:.3}",
            translation.x, translation.y, translation.z
        )
    }
}

struct PlayerMoveEvent;

fn setup_lights(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle::default());
}

fn setup_cameras(mut commands: Commands, player: Res<Player>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform {
            translation: player.transform.translation + Vec3::Y,
            rotation: player.transform.rotation,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(UiCameraBundle::default());
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

fn setup_texts(mut commands: Commands, asset_server: Res<AssetServer>, player: Res<Player>) {
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
            format!("{}", player.into_inner()),
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

fn update(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: ResMut<Player>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
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
        let translation = player.transform.forward() * (move_forward * time.delta_seconds());
        player.transform.translation += translation;
        player_move_events.send(PlayerMoveEvent)
    }

    if turn_left != 0.0 {
        let rotation = Quat::from_rotation_y(turn_left * time.delta_seconds());
        player.transform.rotate(rotation);
    }
}

fn update_cameras(
    player: Res<Player>,
    mut camera_transforms: Query<&mut Transform, With<Camera3d>>,
) {
    if !player.is_changed() {
        return;
    }

    let mut camera_transform = match camera_transforms.get_single_mut() {
        Ok(camera_transform) => camera_transform,
        _ => {
            error!("Transform not found.");
            return;
        }
    };

    camera_transform.translation = player.transform.translation + Vec3::Y;
    camera_transform.rotation = player.transform.rotation;
}

fn update_texts(
    player: Res<Player>,
    mut player_move_events: EventReader<PlayerMoveEvent>,
    mut texts: Query<&mut Text>,
) {
    if player_move_events.iter().count() <= 0 {
        return;
    }

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

    text_section.value = format!("{}", player.into_inner());
}
