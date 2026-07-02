use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

#[derive(Component)]
struct Plant {
    anim_name: String, // "peashooter", "repeater", or "cattail"
    anim_state: PlantAnimState,
    head_frame: usize,
    stem_frame: usize,
    anim_timer: Timer, // Runs at 12 FPS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlantAnimState {
    Idle,
    HeadIdle,
    Shooting,
    FullIdle,
}

#[derive(Component)]
struct ReanimPart {
    track_index: usize,
}

// --- REANIM STRUCTURES ---
struct ReanimFrame {
    x: Option<f32>,
    y: Option<f32>,
    sx: Option<f32>,
    sy: Option<f32>,
    kx: Option<f32>,
    ky: Option<f32>,
    f: Option<i32>,
    i: Option<String>,
}

#[derive(Clone)]
#[allow(dead_code)]
struct ReanimFrameResolved {
    x: f32,
    y: f32,
    sx: f32,
    sy: f32,
    kx: f32,
    ky: f32,
    visible: bool,
    image: Option<String>,
}

struct ReanimTrackResolved {
    name: String,
    frames: Vec<ReanimFrameResolved>,
}

#[derive(Resource)]
struct ReanimLibrary {
    animations: HashMap<String, Vec<ReanimTrackResolved>>,
}

#[derive(Resource)]
struct ReanimTextures {
    handles: HashMap<String, Handle<Image>>,
}

// Mappings from XML resource string to local PNG path
fn get_image_path(resource: &str) -> Option<&'static str> {
    match resource {
        // Peashooter & Repeater parts
        "IMAGE_REANIM_PEASHOOTER_BACKLEAF" => Some("reanim/PeaShooter_backleaf.png"),
        "IMAGE_REANIM_PEASHOOTER_BACKLEAF_LEFTTIP" => Some("reanim/PeaShooter_backleaf_lefttip.png"),
        "IMAGE_REANIM_PEASHOOTER_BACKLEAF_RIGHTTIP" => Some("reanim/PeaShooter_backleaf_righttip.png"),
        "IMAGE_REANIM_PEASHOOTER_BLINK1" => Some("reanim/PeaShooter_blink1.png"),
        "IMAGE_REANIM_PEASHOOTER_BLINK2" => Some("reanim/PeaShooter_blink2.png"),
        "IMAGE_REANIM_PEASHOOTER_EYEBROW" => Some("reanim/PeaShooter_eyebrow.png"),
        "IMAGE_REANIM_PEASHOOTER_FRONTLEAF" => Some("reanim/PeaShooter_frontleaf.png"),
        "IMAGE_REANIM_PEASHOOTER_FRONTLEAF_LEFTTIP" => Some("reanim/PeaShooter_frontleaf_lefttip.png"),
        "IMAGE_REANIM_PEASHOOTER_FRONTLEAF_RIGHTTIP" => Some("reanim/PeaShooter_frontleaf_righttip.png"),
        "IMAGE_REANIM_PEASHOOTER_HEAD" => Some("reanim/PeaShooter_Head.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_2RDFARTHEST" => Some("reanim/PeaShooter_headleaf_2rdfarthest.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_3RDFARTHEST" => Some("reanim/PeaShooter_headleaf_3rdfarthest.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_FARTHEST" => Some("reanim/PeaShooter_headleaf_farthest.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_NEAREST" => Some("reanim/PeaShooter_headleaf_nearest.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_TIP_BOTTOM" => Some("reanim/PeaShooter_headleaf_tip_bottom.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_TIP_TOP" => Some("reanim/PeaShooter_headleaf_tip_top.png"),
        "IMAGE_REANIM_PEASHOOTER_LIPS" => Some("reanim/PeaShooter_Lips.png"),
        "IMAGE_REANIM_PEASHOOTER_MOUTH" => Some("reanim/PeaShooter_mouth.png"),
        "IMAGE_REANIM_PEASHOOTER_STALK_BOTTOM" => Some("reanim/PeaShooter_stalk_bottom.png"),
        "IMAGE_REANIM_PEASHOOTER_STALK_TOP" => Some("reanim/PeaShooter_stalk_top.png"),
        "IMAGE_REANIM_PEASHOOTER_SPROUT" => Some("reanim/PeaShooter_sprout.png"),
        "IMAGE_REANIM_ANIM_SPROUT" => Some("reanim/anim_sprout.png"),
        
        // Cattail parts
        "IMAGE_REANIM_CATTAIL_TAIL2_OVERLAY" => Some("reanim/Cattail_tail2_overlay.png"),
        "IMAGE_REANIM_CATTAIL_SPIKE" => Some("reanim/Cattail_spike.png"),
        "IMAGE_REANIM_CATTAIL_TAIL2" => Some("reanim/Cattail_tail2.png"),
        "IMAGE_REANIM_CATTAIL_PAW1" => Some("reanim/Cattail_paw1.png"),
        "IMAGE_REANIM_CATTAIL_PAW3" => Some("reanim/Cattail_paw3.png"),
        "IMAGE_REANIM_CATTAIL_PAW2" => Some("reanim/Cattail_paw2.png"),
        "IMAGE_REANIM_CATTAIL_BLINK2" => Some("reanim/Cattail_blink2.png"),
        "IMAGE_REANIM_CATTAIL_BLINK1" => Some("reanim/Cattail_blink1.png"),
        "IMAGE_REANIM_CATTAIL_EYEBROW2" => Some("reanim/Cattail_eyebrow2.png"),
        "IMAGE_REANIM_CATTAIL_TAIL" => Some("reanim/Cattail_tail.png"),
        "IMAGE_REANIM_CATTAIL_HEAD" => Some("reanim/Cattail_head.png"),
        "IMAGE_REANIM_CATTAIL_EYEBROW1" => Some("reanim/Cattail_eyebrow1.png"),
        "IMAGE_REANIM_CATTAIL_HAT" => Some("reanim/Cattail_hat.png"),
        "IMAGE_REANIM_CATTAIL_BLINK" => Some("reanim/Cattail_blink.png"),
        _ => None,
    }
}

// Custom simple XML parser
fn parse_tag_str(s: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    if let Some(start_idx) = s.find(&start_tag) {
        if let Some(end_idx) = s.find(&end_tag) {
            return Some(s[start_idx + start_tag.len()..end_idx].to_string());
        }
    }
    None
}

fn parse_tag_f32(s: &str, tag: &str) -> Option<f32> {
    parse_tag_str(s, tag).and_then(|v| v.parse::<f32>().ok())
}

fn parse_tag_i32(s: &str, tag: &str) -> Option<i32> {
    parse_tag_str(s, tag).and_then(|v| v.parse::<i32>().ok())
}

fn parse_reanim(content: &str) -> Vec<ReanimTrackResolved> {
    let mut tracks = Vec::new();
    let parts: Vec<&str> = content.split("<track>").collect();
    for part in parts.iter().skip(1) {
        let name = if let Some(n_start) = part.find("<name>") {
            if let Some(n_end) = part.find("</name>") {
                part[n_start + 6..n_end].to_string()
            } else {
                continue;
            }
        } else {
            continue;
        };

        let mut raw_frames = Vec::new();
        let t_parts: Vec<&str> = part.split("<t>").collect();
        for t_part in t_parts.iter().skip(1) {
            let t_content = if let Some(t_end) = t_part.find("</t>") {
                &t_part[..t_end]
            } else {
                t_part
            };

            let x = parse_tag_f32(t_content, "x");
            let y = parse_tag_f32(t_content, "y");
            let sx = parse_tag_f32(t_content, "sx");
            let sy = parse_tag_f32(t_content, "sy");
            let kx = parse_tag_f32(t_content, "kx");
            let ky = parse_tag_f32(t_content, "ky");
            let f = parse_tag_i32(t_content, "f");
            let i = parse_tag_str(t_content, "i");

            raw_frames.push(ReanimFrame { x, y, sx, sy, kx, ky, f, i });
        }

        let mut resolved_frames = Vec::new();
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        let mut current_sx = 1.0;
        let mut current_sy = 1.0;
        let mut current_kx = 0.0;
        let mut current_ky = 0.0;
        let mut current_f = -1;
        let mut current_i = None;

        for frame in raw_frames {
            if let Some(val) = frame.x { current_x = val; }
            if let Some(val) = frame.y { current_y = val; }
            if let Some(val) = frame.sx { current_sx = val; }
            if let Some(val) = frame.sy { current_sy = val; }
            if let Some(val) = frame.kx { current_kx = val; }
            if let Some(val) = frame.ky { current_ky = val; }
            if let Some(val) = frame.f { current_f = val; }
            if let Some(ref val) = frame.i { current_i = Some(val.clone()); }

            resolved_frames.push(ReanimFrameResolved {
                x: current_x,
                y: current_y,
                sx: current_sx,
                sy: current_sy,
                kx: current_kx,
                ky: current_ky,
                visible: current_f >= 0,
                image: current_i.clone(),
            });
        }

        tracks.push(ReanimTrackResolved { name, frames: resolved_frames });
    }
    tracks
}

fn main() {
    // 1. Read and parse Peashooter (Single)
    let single_xml = fs::read_to_string("assets/reanim/PeaShooterSingle.reanim")
        .expect("Failed to read PeaShooterSingle.reanim file.");
    let single_tracks = parse_reanim(&single_xml);

    // 2. Read and parse Repeater (Mohawk head leaves)
    let repeater_xml = fs::read_to_string("assets/reanim/PeaShooter.reanim")
        .expect("Failed to read PeaShooter.reanim file.");
    let repeater_tracks = parse_reanim(&repeater_xml);

    // 3. Read and parse Cattail
    let cattail_xml = fs::read_to_string("assets/reanim/Cattail.reanim")
        .expect("Failed to read Cattail.reanim file.");
    let cattail_tracks = parse_reanim(&cattail_xml);

    let mut library = HashMap::new();
    library.insert("peashooter".to_string(), single_tracks);
    library.insert("repeater".to_string(), repeater_tracks);
    library.insert("cattail".to_string(), cattail_tracks);

    let asset_path = if std::path::Path::new("pvz_logic/assets").exists() {
        "pvz_logic/assets".to_string()
    } else {
        "assets".to_string()
    };

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "PvZ Plant Animation Debugger (Peashooter, Repeater, Cattail)".into(),
                    resolution: (1000.0, 600.0).into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: asset_path,
                ..default()
            })
        )
        .insert_resource(ReanimLibrary { animations: library })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            tick_timers,
            animate_plant,
            handle_keyboard_triggers,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    reanim_library: Res<ReanimLibrary>,
) {
    // 2D Camera
    commands.spawn(Camera2dBundle::default());

    // Center grid lines for layout debug
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.05),
            custom_size: Some(Vec2::new(1000.0, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Load all the required textures and store them
    let mut textures_map = HashMap::new();
    for tracks in reanim_library.animations.values() {
        for track in tracks {
            for frame in &track.frames {
                if let Some(ref img_res) = frame.image {
                    if !textures_map.contains_key(img_res) {
                        if let Some(file_path) = get_image_path(img_res) {
                            let handle = asset_server.load(file_path);
                            textures_map.insert(img_res.clone(), handle);
                        }
                    }
                }
            }
        }
    }
    commands.insert_resource(ReanimTextures { handles: textures_map });

    // Spawn 1. PeaShooter (Single) on the left (X = -300)
    let peashooter_entity = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-300.0, -50.0, 2.0).with_scale(Vec3::splat(1.5)),
            ..default()
        },
        Plant {
            anim_name: "peashooter".to_string(),
            anim_state: PlantAnimState::Idle,
            head_frame: 79,
            stem_frame: 79,
            anim_timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    )).id();

    // Spawn PeaShooter parts
    for (idx, track) in reanim_library.animations.get("peashooter").unwrap().iter().enumerate() {
        if track.name == "anim_idle"
            || track.name == "anim_shooting"
            || track.name == "anim_head_idle"
            || track.name == "anim_full_idle"
        {
            continue;
        }

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            ReanimPart { track_index: idx },
        )).set_parent(peashooter_entity);
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Peashooter (Single)\n[PeaShooterSingle.reanim]",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.7, 1.0, 0.7),
                ..default()
            },
        ).with_justify(JustifyText::Center),
        transform: Transform::from_xyz(-300.0, -120.0, 3.0),
        ..default()
    });


    // Spawn 2. Repeater in the center (X = 0)
    let repeater_entity = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, -50.0, 2.0).with_scale(Vec3::splat(1.5)),
            ..default()
        },
        Plant {
            anim_name: "repeater".to_string(),
            anim_state: PlantAnimState::Idle,
            head_frame: 79,
            stem_frame: 79,
            anim_timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    )).id();

    // Spawn Repeater parts
    for (idx, track) in reanim_library.animations.get("repeater").unwrap().iter().enumerate() {
        if track.name == "anim_idle"
            || track.name == "anim_shooting"
            || track.name == "anim_head_idle"
            || track.name == "anim_full_idle"
        {
            continue;
        }

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            ReanimPart { track_index: idx },
        )).set_parent(repeater_entity);
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Repeater\n[PeaShooter.reanim]",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.5, 0.9, 0.9),
                ..default()
            },
        ).with_justify(JustifyText::Center),
        transform: Transform::from_xyz(0.0, -120.0, 3.0),
        ..default()
    });


    // Spawn 3. Cattail on the right (X = 300)
    let cattail_entity = commands.spawn((
        SpatialBundle {
            // Offset slightly to center the cat body nicely
            transform: Transform::from_xyz(260.0, -20.0, 2.0).with_scale(Vec3::splat(1.5)),
            ..default()
        },
        Plant {
            anim_name: "cattail".to_string(),
            anim_state: PlantAnimState::Idle,
            head_frame: 5,
            stem_frame: 5,
            anim_timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    )).id();

    // Spawn Cattail parts
    for (idx, track) in reanim_library.animations.get("cattail").unwrap().iter().enumerate() {
        if track.name == "anim_idle"
            || track.name == "anim_shooting"
            || track.name == "anim_blink"
            || track.name == "anim_head_idle"
            || track.name == "anim_full_idle"
        {
            continue;
        }

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            ReanimPart { track_index: idx },
        )).set_parent(cattail_entity);
    }

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Cattail\n[Cattail.reanim]",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(1.0, 0.7, 0.8),
                ..default()
            },
        ).with_justify(JustifyText::Center),
        transform: Transform::from_xyz(300.0, -120.0, 3.0),
        ..default()
    });


    // On-screen UI instructions
    commands.spawn(
        TextBundle::from_section(
            "PvZ Plant Animation Debugger\n\
             ============================\n\n\
             Assembled side-by-side using TopLeft coordinate anchoring.\n\n\
             Key triggers:\n\
             [Press 1]: Trigger Idle Animation (All loop their Idle/FullIdle frames)\n\
             [Press 2]: Trigger Head Idle Animation (Loops HeadIdle for Peas; loops Idle for Cattail)\n\
             [Press 3]: Trigger Shooting Animation (Loops Shooting: Peas 54-78, Cattail 24-39)\n\
             [Press 4]: Trigger Full Idle Animation (All loop their Idle/FullIdle frames)",
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

fn tick_timers(time: Res<Time>, mut plant_query: Query<&mut Plant>) {
    let delta = time.delta();
    for mut plant in &mut plant_query {
        plant.anim_timer.tick(delta);
    }
}

// Animation system for all plants (Peashooter, Repeater, Cattail)
fn animate_plant(
    reanim_library: Res<ReanimLibrary>,
    reanim_textures: Res<ReanimTextures>,
    mut plant_query: Query<(Entity, &mut Plant)>,
    parent_query: Query<&Children>,
    mut part_query: Query<(&ReanimPart, &mut Sprite, &mut Transform, &mut Visibility, &mut Handle<Image>)>,
) {
    for (plant_entity, mut plant) in &mut plant_query {
        if plant.anim_timer.just_finished() {
            // Update frames based on plant name
            if plant.anim_name == "cattail" {
                // Cattail frame bounds:
                // Idle = 5..=23
                // Shooting = 24..=39
                
                // 1. Update stem/base frame (always loops idle 5..=23)
                plant.stem_frame += 1;
                if plant.stem_frame > 23 {
                    plant.stem_frame = 5;
                }

                // 2. Update head/active frame
                match plant.anim_state {
                    PlantAnimState::Idle | PlantAnimState::FullIdle | PlantAnimState::HeadIdle => {
                        plant.head_frame = plant.stem_frame;
                    }
                    PlantAnimState::Shooting => {
                        plant.head_frame += 1;
                        if plant.head_frame > 39 || plant.head_frame < 24 {
                            plant.head_frame = 24;
                        }
                    }
                }
            } else {
                // Peas frame bounds:
                // Stem Idle = 79..=103 (or 4..=28)
                // Head Idle = 29..=53
                // Shooting = 54..=78
                
                // 1. Update stem frame (always loops full idle 79-103)
                plant.stem_frame += 1;
                if plant.stem_frame > 103 {
                    plant.stem_frame = 79;
                }

                // 2. Update head frame based on state
                match plant.anim_state {
                    PlantAnimState::Idle | PlantAnimState::FullIdle => {
                        plant.head_frame = plant.stem_frame;
                    }
                    PlantAnimState::HeadIdle => {
                        plant.head_frame += 1;
                        if plant.head_frame > 53 || plant.head_frame < 29 {
                            plant.head_frame = 29;
                        }
                    }
                    PlantAnimState::Shooting => {
                        plant.head_frame += 1;
                        if plant.head_frame > 78 || plant.head_frame < 54 {
                            plant.head_frame = 54;
                        }
                    }
                }
            }
        }

        // Apply animations to parts (children)
        if let Ok(children) = parent_query.get(plant_entity) {
            if let Some(tracks) = reanim_library.animations.get(&plant.anim_name) {
                for &child in children {
                    if let Ok((part, mut sprite, mut transform, mut visibility, mut texture)) = part_query.get_mut(child) {
                        let track = &tracks[part.track_index];
                        
                        let head_frame_data = if plant.head_frame < track.frames.len() {
                            Some(&track.frames[plant.head_frame])
                        } else {
                            None
                        };

                        let stem_frame_data = if plant.stem_frame < track.frames.len() {
                            Some(&track.frames[plant.stem_frame])
                        } else {
                            None
                        };

                        // Blending Rule: If visible in active head frame, use it; otherwise fallback to stem frame
                        let target_frame_data = if let Some(head_data) = head_frame_data {
                            if head_data.visible {
                                Some(head_data)
                            } else {
                                stem_frame_data
                            }
                        } else {
                            stem_frame_data
                        };

                        if let Some(frame_data) = target_frame_data {
                            if frame_data.visible {
                                *visibility = Visibility::Inherited;
                                
                                if let Some(ref img_res) = frame_data.image {
                                    if let Some(handle) = reanim_textures.handles.get(img_res) {
                                        *texture = handle.clone();
                                        sprite.custom_size = None;
                                    }
                                }
                                
                                // Align position
                                transform.translation.x = frame_data.x;
                                transform.translation.y = -frame_data.y;
                                transform.translation.z = part.track_index as f32 * 0.01;
                                
                                // Scale
                                transform.scale.x = frame_data.sx;
                                transform.scale.y = frame_data.sy;
                                
                                // Rotate around local origin (TopLeft anchor)
                                transform.rotation = Quat::from_rotation_z(-frame_data.kx.to_radians());
                            } else {
                                *visibility = Visibility::Hidden;
                            }
                        } else {
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
            }
        }
    }
}

// Keyboard input debug trigger
fn handle_keyboard_triggers(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut plant_query: Query<&mut Plant>,
) {
    for mut plant in &mut plant_query {
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            plant.anim_state = PlantAnimState::Idle;
            if plant.anim_name == "cattail" {
                plant.head_frame = 5;
                plant.stem_frame = 5;
            } else {
                plant.head_frame = 79;
                plant.stem_frame = 79;
            }
            info!("Switched {} state to Idle", plant.anim_name);
        }
        if keyboard_input.just_pressed(KeyCode::Digit2) {
            plant.anim_state = PlantAnimState::HeadIdle;
            if plant.anim_name == "cattail" {
                plant.head_frame = 5;
                plant.stem_frame = 5;
            } else {
                plant.head_frame = 29;
                plant.stem_frame = 79;
            }
            info!("Switched {} state to Head Idle", plant.anim_name);
        }
        if keyboard_input.just_pressed(KeyCode::Digit3) {
            plant.anim_state = PlantAnimState::Shooting;
            if plant.anim_name == "cattail" {
                plant.head_frame = 24;
                plant.stem_frame = 5;
            } else {
                plant.head_frame = 54;
                plant.stem_frame = 79;
            }
            info!("Switched {} state to Shooting", plant.anim_name);
        }
        if keyboard_input.just_pressed(KeyCode::Digit4) {
            plant.anim_state = PlantAnimState::FullIdle;
            if plant.anim_name == "cattail" {
                plant.head_frame = 5;
                plant.stem_frame = 5;
            } else {
                plant.head_frame = 79;
                plant.stem_frame = 79;
            }
            info!("Switched {} state to Full Idle", plant.anim_name);
        }
    }
}
