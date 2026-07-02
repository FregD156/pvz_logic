use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::f32::consts::PI;

#[derive(Resource)]
struct RollState {
    progress: f32,
    active: bool,
    roll_type: usize, // 1, 3, or 5 rows
    speed: f32,       // Progress per second
}

#[derive(Component)]
struct UnsortedBackground;

#[derive(Component)]
struct SodLayer;

#[derive(Component)]
struct SodCylinder;

#[derive(Component)]
struct SodCap {
    is_top: bool,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PvZ Grass Sod Roll Simulator".into(),
                resolution: (1400.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(RollState {
            progress: 0.0,
            active: true,
            roll_type: 5,
            speed: 0.4, // Rolls out completely in 2.5 seconds
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            tick_roll,
            update_sod_rendering,
            handle_inputs,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 2D Camera
    commands.spawn(Camera2dBundle::default());

    // Spawn Unsodded base background (Z = 0.0)
    let unsodded_handle = asset_server.load("images/background1unsodded.jpg");
    commands.spawn((
        SpriteBundle {
            texture: unsodded_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        UnsortedBackground,
    ));

    // Spawn Sod Layer (Z = 1.0)
    let sod_handle = asset_server.load("images/background1.jpg");
    commands.spawn((
        SpriteBundle {
            texture: sod_handle,
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_xyz(-700.0, 0.0, 1.0),
            ..default()
        },
        SodLayer,
    ));

    // Spawn Sod Roll Cylinder (Z = 2.0)
    let cylinder_handle = asset_server.load("reanim/SodRoll.png");
    commands.spawn((
        SpriteBundle {
            texture: cylinder_handle,
            transform: Transform::from_xyz(-700.0, 0.0, 2.0),
            ..default()
        },
        SodCylinder,
    ));

    // Spawn Sod Roll Cap (Top) (Z = 2.1)
    let cap_handle = asset_server.load("reanim/SodRollCap.png");
    commands.spawn((
        SpriteBundle {
            texture: cap_handle.clone(),
            transform: Transform::from_xyz(-700.0, 0.0, 2.1),
            ..default()
        },
        SodCap { is_top: true },
    ));

    // Spawn Sod Roll Cap (Bottom) (Z = 2.1)
    commands.spawn((
        SpriteBundle {
            texture: cap_handle,
            transform: Transform::from_xyz(-700.0, 0.0, 2.1),
            ..default()
        },
        SodCap { is_top: false },
    ));

    // Play rolling audio immediately
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/roll_in.ogg"),
        ..default()
    });

    // UI Overlay Information
    commands.spawn(
        TextBundle::from_section(
            "PvZ Grass Sod Roll Simulator\n\
             ============================\n\n\
             Key Triggers:\n\
             [Press Space]: Reset and Play Sod Roll again\n\
             [Press 1]: Roll 1-Row Sod (Tutorial Level 1-1 style)\n\
             [Press 3]: Roll 3-Row Sod (Tutorial Level 1-2 style)\n\
             [Press 5]: Roll Full 5-Row Lawn (Classic Backyard style)\n\n\
             Status: Rolling Full 5-Row Lawn...",
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
}

fn tick_roll(
    time: Res<Time>,
    mut state: ResMut<RollState>,
) {
    if state.active {
        state.progress += state.speed * time.delta_seconds();
        if state.progress >= 1.0 {
            state.progress = 1.0;
            state.active = false;
        }
    }
}

fn update_sod_rendering(
    state: Res<RollState>,
    asset_server: Res<AssetServer>,
    mut q_sod: Query<(&mut Handle<Image>, &mut Sprite, &mut Transform), (With<SodLayer>, Without<SodCylinder>, Without<SodCap>)>,
    mut q_cylinder: Query<(&mut Transform, &mut Visibility), (With<SodCylinder>, Without<SodLayer>, Without<SodCap>)>,
    mut q_caps: Query<(&mut Transform, &mut Visibility, &SodCap), (With<SodCap>, Without<SodLayer>, Without<SodCylinder>)>,
) {
    let p = state.progress;

    // 1. Get Sod properties based on roll type
    let (texture_path, left_x, total_width, cell_height, center_y) = match state.roll_type {
        1 => ("images/sod1row.jpg", -360.0, 771.0, 110.0, -15.0),
        3 => ("images/sod3row.jpg", -360.0, 771.0, 310.0, -15.0),
        _ => ("images/background1.jpg", -700.0, 1400.0, 480.0, 0.0),
    };

    // 2. Render Crop and Expansion
    if let Ok((mut texture, mut sprite, mut transform)) = q_sod.get_single_mut() {
        *texture = asset_server.load(texture_path);
        
        transform.translation.x = left_x;
        transform.translation.y = center_y;

        let visible_w = total_width * p;
        sprite.rect = Some(Rect {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(visible_w, 600.0),
        });
    }

    // 3. Cylinder roll and rotation
    let lead_x = left_x + total_width * p;
    let is_rolling = state.active && p < 1.0;

    if let Ok((mut transform, mut visibility)) = q_cylinder.get_single_mut() {
        if is_rolling {
            *visibility = Visibility::Inherited;
            transform.translation.x = lead_x;
            transform.translation.y = center_y;
            
            // Scale cylinder to cover the sod height (SodRoll.png is 68x141)
            transform.scale.y = cell_height / 141.0;
            transform.scale.x = 1.0;
            
            // Rotate continuously
            transform.rotation = Quat::from_rotation_z(-p * 10.0 * PI);
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    // 4. Side Caps positioning
    for (mut transform, mut visibility, cap) in &mut q_caps {
        if is_rolling {
            *visibility = Visibility::Inherited;
            
            // Place top and bottom caps at the ends of the cylinder
            let cap_y = if cap.is_top {
                center_y + cell_height / 2.0
            } else {
                center_y - cell_height / 2.0
            };
            
            transform.translation.x = lead_x;
            transform.translation.y = cap_y;
            
            // SodRollCap.png is 73x71 (nearly round), so we scale it symmetrically
            transform.scale = Vec3::splat(1.1);
            transform.rotation = Quat::from_rotation_z(-p * 10.0 * PI);
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn handle_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<RollState>,
    mut text_query: Query<&mut Text>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let mut triggered = false;

    if keyboard_input.just_pressed(KeyCode::Space) {
        state.progress = 0.0;
        state.active = true;
        triggered = true;
    }

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        state.roll_type = 1;
        state.progress = 0.0;
        state.active = true;
        triggered = true;
    }

    if keyboard_input.just_pressed(KeyCode::Digit3) {
        state.roll_type = 3;
        state.progress = 0.0;
        state.active = true;
        triggered = true;
    }

    if keyboard_input.just_pressed(KeyCode::Digit5) {
        state.roll_type = 5;
        state.progress = 0.0;
        state.active = true;
        triggered = true;
    }

    if triggered {
        // Re-play roll audio
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/roll_in.ogg"),
            ..default()
        });

        if let Ok(mut text) = text_query.get_single_mut() {
            let label = match state.roll_type {
                1 => "Rolling 1-Row Sod (Level 1-1)...",
                3 => "Rolling 3-Row Sod (Level 1-2 & 1-3)...",
                _ => "Rolling Full 5-Row Lawn...",
            };
            text.sections[0].value = format!(
                "PvZ Grass Sod Roll Simulator\n\
                 ============================\n\n\
                 Key Triggers:\n\
                 [Press Space]: Reset and Play Sod Roll again\n\
                 [Press 1]: Roll 1-Row Sod (Tutorial Level 1-1 style)\n\
                 [Press 3]: Roll 3-Row Sod (Tutorial Level 1-2 style)\n\
                 [Press 5]: Roll Full 5-Row Lawn (Classic Backyard style)\n\n\
                 Status: {}",
                label
            );
        }
    }
}
