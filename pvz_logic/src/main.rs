use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

// Spatial configuration
const LAWN_LEFT: f32 = -400.0;
const LAWN_RIGHT: f32 = 400.0;
const LAWN_Y: f32 = 0.0;

// Coordinate mapping: 0 to 1000 maps to LAWN_LEFT to LAWN_RIGHT
fn map_x_to_screen(x: f32) -> f32 {
    LAWN_LEFT + (x / 1000.0) * (LAWN_RIGHT - LAWN_LEFT)
}

// Configuration constants
const PLANT_X: f32 = 200.0; // Plant position at X=200
const ZOMBIE_SPAWN_X: f32 = 1000.0; // Zombie spawn at X=1000
const ZOMBIE_WALK_SPEED: f32 = 40.0;
const BULLET_SPEED: f32 = 250.0;
const BULLET_DAMAGE: f32 = 20.0;
const PLANT_MAX_HP: f32 = 300.0;
const ZOMBIE_MAX_HP: f32 = 100.0;
const PLANT_SHOOT_COOLDOWN: f32 = 2.0;
const ZOMBIE_EAT_COOLDOWN: f32 = 0.5;
const ZOMBIE_EAT_DAMAGE: f32 = 20.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

// Components
#[derive(Component)]
struct XPosition(f32);

#[derive(Component)]
struct Health {
    hp: f32,
    max_hp: f32,
}

#[derive(Component)]
struct Plant {
    shoot_timer: Timer,
    anim_state: PlantAnimState,
    head_frame: usize,
    stem_frame: usize,
    anim_timer: Timer, // Runs at 12 FPS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlantAnimState {
    Idle,
    Shooting,
}

#[derive(Component)]
struct ReanimPart {
    track_index: usize,
}

#[derive(Component)]
struct Bullet {
    damage: f32,
}

#[derive(Component)]
struct Zombie {
    speed: f32,
    state: ZombieState,
    eat_timer: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ZombieState {
    Walking,
    Eating,
}

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct HealthBarFill;

// Resource for periodic zombie spawning
#[derive(Resource)]
struct ZombieSpawnTimer(Timer);

// Resource for text information display
#[derive(Component)]
struct InfoText;

// --- REANIM STRUCTURES ---
#[allow(dead_code)]
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
struct ReanimData {
    tracks: Vec<ReanimTrackResolved>,
}

#[derive(Resource)]
struct ReanimTextures {
    handles: HashMap<String, Handle<Image>>,
}

// Mappings from XML resource string to local PNG path
fn get_image_path(resource: &str) -> Option<&'static str> {
    match resource {
        "IMAGE_REANIM_PEASHOOTER_BACKLEAF" => Some("PvZ_Assets/reanim/PeaShooter_backleaf.png"),
        "IMAGE_REANIM_PEASHOOTER_BACKLEAF_LEFTTIP" => Some("PvZ_Assets/reanim/PeaShooter_backleaf_lefttip.png"),
        "IMAGE_REANIM_PEASHOOTER_BACKLEAF_RIGHTTIP" => Some("PvZ_Assets/reanim/PeaShooter_backleaf_righttip.png"),
        "IMAGE_REANIM_PEASHOOTER_BLINK1" => Some("PvZ_Assets/reanim/PeaShooter_blink1.png"),
        "IMAGE_REANIM_PEASHOOTER_BLINK2" => Some("PvZ_Assets/reanim/PeaShooter_blink2.png"),
        "IMAGE_REANIM_PEASHOOTER_EYEBROW" => Some("PvZ_Assets/reanim/PeaShooter_eyebrow.png"),
        "IMAGE_REANIM_PEASHOOTER_FRONTLEAF" => Some("PvZ_Assets/reanim/PeaShooter_frontleaf.png"),
        "IMAGE_REANIM_PEASHOOTER_FRONTLEAF_LEFTTIP" => Some("PvZ_Assets/reanim/PeaShooter_frontleaf_lefttip.png"),
        "IMAGE_REANIM_PEASHOOTER_FRONTLEAF_RIGHTTIP" => Some("PvZ_Assets/reanim/PeaShooter_frontleaf_righttip.png"),
        "IMAGE_REANIM_PEASHOOTER_HEAD" => Some("PvZ_Assets/reanim/PeaShooter_Head.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_2RDFARTHEST" => Some("PvZ_Assets/reanim/PeaShooter_headleaf_2rdfarthest.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_3RDFARTHEST" => Some("PvZ_Assets/reanim/PeaShooter_headleaf_3rdfarthest.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_FARTHEST" => Some("PvZ_Assets/reanim/PeaShooter_headleaf_farthest.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_NEAREST" => Some("PvZ_Assets/reanim/PeaShooter_headleaf_nearest.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_TIP_BOTTOM" => Some("PvZ_Assets/reanim/PeaShooter_headleaf_tip_bottom.png"),
        "IMAGE_REANIM_PEASHOOTER_HEADLEAF_TIP_TOP" => Some("PvZ_Assets/reanim/PeaShooter_headleaf_tip_top.png"),
        "IMAGE_REANIM_PEASHOOTER_LIPS" => Some("PvZ_Assets/reanim/PeaShooter_Lips.png"),
        "IMAGE_REANIM_PEASHOOTER_MOUTH" => Some("PvZ_Assets/reanim/PeaShooter_mouth.png"),
        "IMAGE_REANIM_PEASHOOTER_STALK_BOTTOM" => Some("PvZ_Assets/reanim/PeaShooter_stalk_bottom.png"),
        "IMAGE_REANIM_PEASHOOTER_STALK_TOP" => Some("PvZ_Assets/reanim/PeaShooter_stalk_top.png"),
        "IMAGE_REANIM_PEASHOOTER_SPROUT" => Some("PvZ_Assets/reanim/PeaShooter_sprout.png"),
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
    // Read and parse reanim file at setup time
    let xml_content = fs::read_to_string("assets/PvZ_Assets/reanim/PeaShooter.reanim")
        .expect("Failed to read PeaShooter.reanim file. Ensure assets symlink is set up!");
    let parsed_tracks = parse_reanim(&xml_content);
    println!("Successfully parsed {} tracks from PeaShooter.reanim", parsed_tracks.len());

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Plants vs. Zombies - Animated Peashooter Showcase".into(),
                resolution: (1000.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(ReanimData { tracks: parsed_tracks })
        .insert_resource(ZombieSpawnTimer(Timer::from_seconds(8.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            tick_timers,
            spawn_zombies,
            zombie_movement,
            plant_shooting,
            animate_plant,
            bullet_movement,
            bullet_collision,
            zombie_attack,
            zombie_eating,
            plant_death,
            sync_transforms,
            update_health_bars,
            update_info_text,
            handle_keyboard_triggers,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(Update, check_game_over.run_if(in_state(GameState::Playing)))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    reanim_data: Res<ReanimData>,
) {
    // 2D Camera
    commands.spawn(Camera2dBundle::default());

    // Background lawn line
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.12, 0.38, 0.12),
            custom_size: Some(Vec2::new(LAWN_RIGHT - LAWN_LEFT + 100.0, 160.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, LAWN_Y - 40.0, 0.0),
        ..default()
    });

    // Marker lines
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.2, 0.5, 0.2),
            custom_size: Some(Vec2::new(10.0, 180.0)),
            ..default()
        },
        transform: Transform::from_xyz(map_x_to_screen(PLANT_X), LAWN_Y - 40.0, 1.0),
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.6, 0.2, 0.2),
            custom_size: Some(Vec2::new(10.0, 180.0)),
            ..default()
        },
        transform: Transform::from_xyz(map_x_to_screen(ZOMBIE_SPAWN_X), LAWN_Y - 40.0, 1.0),
        ..default()
    });

    // Load all the required Peashooter textures and store them
    let mut textures_map = HashMap::new();
    for track in &reanim_data.tracks {
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
    commands.insert_resource(ReanimTextures { handles: textures_map });

    // Spawn the Plant at X=200
    spawn_plant(&mut commands, PLANT_X, &reanim_data);

    // Initial Zombie at X=1000
    spawn_zombie_at(&mut commands, ZOMBIE_SPAWN_X);

    // On-screen UI instructions
    commands.spawn((
        TextBundle::from_section(
            "Plants vs. Zombies - Bevy Animated Peashooter\n\
             Assembled from 19 parts in PeaShooter.reanim\n\n\
             Key triggers:\n\
             [Press 1]: Trigger Idle Animation (frames 4-28)\n\
             [Press 2]: Trigger Head Idle Animation (frames 29-53)\n\
             [Press 3]: Trigger Shooting Animation (frames 54-78)\n\
             [Press 4]: Trigger Full Idle Animation (frames 79-103)\n\n\
             Peashooter shoots a pea when Zombie enters range!",
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
        InfoText,
    ));
}

fn spawn_plant(commands: &mut Commands, x: f32, reanim_data: &ReanimData) {
    // Spawn the parent plant entity
    // We position the parent offset so that the bottom of the stalk sits correctly on lawn
    let plant_entity = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(map_x_to_screen(x) - 30.0, LAWN_Y + 60.0, 2.0),
            ..default()
        },
        XPosition(x),
        Health {
            hp: PLANT_MAX_HP,
            max_hp: PLANT_MAX_HP,
        },
        Plant {
            shoot_timer: Timer::from_seconds(PLANT_SHOOT_COOLDOWN, TimerMode::Repeating),
            anim_state: PlantAnimState::Idle,
            head_frame: 79,
            stem_frame: 79,
            anim_timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    )).id();

    // Spawn the body parts as children
    for (idx, track) in reanim_data.tracks.iter().enumerate() {
        // Skip controllers
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
        )).set_parent(plant_entity);
    }

    // Spawn Plant's health bar
    spawn_health_bar(commands, plant_entity, 70.0);
}

fn spawn_zombie_at(commands: &mut Commands, x: f32) {
    let zombie_entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.3, 0.6),
                custom_size: Some(Vec2::new(45.0, 80.0)),
                ..default()
            },
            transform: Transform::from_xyz(map_x_to_screen(x), LAWN_Y - 20.0, 2.0),
            ..default()
        },
        XPosition(x),
        Health {
            hp: ZOMBIE_MAX_HP,
            max_hp: ZOMBIE_MAX_HP,
        },
        Zombie {
            speed: ZOMBIE_WALK_SPEED,
            state: ZombieState::Walking,
            eat_timer: Timer::from_seconds(ZOMBIE_EAT_COOLDOWN, TimerMode::Repeating),
        },
    )).id();

    spawn_health_bar(commands, zombie_entity, 50.0);
}

fn spawn_health_bar(commands: &mut Commands, parent: Entity, y_offset: f32) {
    let bar_bg = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(50.0, 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, y_offset, 1.0),
            ..default()
        },
        HealthBar,
    )).id();

    let bar_fill = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.1, 0.1),
                custom_size: Some(Vec2::new(50.0, 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        HealthBarFill,
    )).id();

    commands.entity(bar_bg).add_child(bar_fill);
    commands.entity(parent).add_child(bar_bg);
}

fn tick_timers(
    time: Res<Time>,
    mut spawn_timer: ResMut<ZombieSpawnTimer>,
    mut plant_query: Query<&mut Plant>,
    mut zombie_query: Query<&mut Zombie>,
) {
    let delta = time.delta();
    spawn_timer.0.tick(delta);

    for mut plant in &mut plant_query {
        plant.shoot_timer.tick(delta);
        plant.anim_timer.tick(delta);
    }

    for mut zombie in &mut zombie_query {
        if zombie.state == ZombieState::Eating {
            zombie.eat_timer.tick(delta);
        }
    }
}

fn spawn_zombies(mut commands: Commands, spawn_timer: Res<ZombieSpawnTimer>) {
    if spawn_timer.0.just_finished() {
        spawn_zombie_at(&mut commands, ZOMBIE_SPAWN_X);
        info!("A new Zombie spawned at X={}", ZOMBIE_SPAWN_X);
    }
}

fn zombie_movement(time: Res<Time>, mut query: Query<(&mut XPosition, &Zombie)>) {
    for (mut pos, zombie) in &mut query {
        if zombie.state == ZombieState::Walking {
            pos.0 -= zombie.speed * time.delta_seconds();
            if pos.0 < 0.0 {
                pos.0 = 0.0;
            }
        }
    }
}

// Plant shooting logic (triggers animation state)
fn plant_shooting(
    mut commands: Commands,
    mut plant_query: Query<(&XPosition, &mut Plant)>,
    zombie_query: Query<&XPosition, With<Zombie>>,
) {
    for (plant_pos, mut plant) in &mut plant_query {
        let zombie_in_range = zombie_query.iter().any(|z_pos| z_pos.0 > plant_pos.0);

        if zombie_in_range {
            if plant.shoot_timer.finished() {
                // Spawn a new Pea Bullet entity
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.9, 0.9, 0.1),
                            custom_size: Some(Vec2::new(14.0, 14.0)),
                            ..default()
                        },
                        // Position slightly higher to match Peashooter mouth height
                        transform: Transform::from_xyz(map_x_to_screen(plant_pos.0) + 10.0, LAWN_Y + 10.0, 3.0),
                        ..default()
                    },
                    XPosition(plant_pos.0 + 10.0),
                    Bullet {
                        damage: BULLET_DAMAGE,
                    },
                ));
                
                // Trigger Shooting animation
                plant.anim_state = PlantAnimState::Shooting;
                plant.head_frame = 54;
                
                plant.shoot_timer.reset();
                info!("Peashooter shot a bullet and triggered Shooting anim!");
            }
        } else {
            plant.shoot_timer.reset();
        }
    }
}

// Animation system for Peashooter
fn animate_plant(
    reanim_data: Res<ReanimData>,
    reanim_textures: Res<ReanimTextures>,
    mut plant_query: Query<(Entity, &mut Plant)>,
    parent_query: Query<&Children>,
    mut part_query: Query<(&ReanimPart, &mut Sprite, &mut Transform, &mut Visibility, &mut Handle<Image>)>,
) {
    for (plant_entity, mut plant) in &mut plant_query {
        if plant.anim_timer.just_finished() {
            // 1. Update stem frame (always loops full idle 79-103)
            plant.stem_frame += 1;
            if plant.stem_frame > 103 {
                plant.stem_frame = 79;
            }

            // 2. Update head frame based on animation state
            match plant.anim_state {
                PlantAnimState::Idle => {
                    plant.head_frame = plant.stem_frame;
                }
                PlantAnimState::Shooting => {
                    plant.head_frame += 1;
                    if plant.head_frame > 78 || plant.head_frame < 54 {
                        plant.anim_state = PlantAnimState::Idle; // Go back to Idle after shoot completes
                        plant.head_frame = plant.stem_frame;
                    }
                }
            }
        }

        // Apply animations to parts (children)
        if let Ok(children) = parent_query.get(plant_entity) {
            for &child in children {
                if let Ok((part, mut sprite, mut transform, mut visibility, mut texture)) = part_query.get_mut(child) {
                    let track = &reanim_data.tracks[part.track_index];
                    
                    // Fetch head frame data
                    let head_frame_data = if plant.head_frame < track.frames.len() {
                        Some(&track.frames[plant.head_frame])
                    } else {
                        None
                    };

                    // Fetch stem frame data
                    let stem_frame_data = if plant.stem_frame < track.frames.len() {
                        Some(&track.frames[plant.stem_frame])
                    } else {
                        None
                    };

                    // Determine which frame data to use:
                    // If the track is visible in the head frame, we use the head frame (Shooting / FullIdle).
                    // If the track is NOT visible in the head frame, it falls back to the stem frame (FullIdle).
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
                            transform.translation.x = frame_data.x * 0.8;
                            transform.translation.y = -frame_data.y * 0.8;
                            transform.translation.z = part.track_index as f32 * 0.01;
                            
                            // Scale
                            transform.scale.x = frame_data.sx * 0.8;
                            transform.scale.y = frame_data.sy * 0.8;
                            
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

fn bullet_movement(time: Res<Time>, mut query: Query<&mut XPosition, With<Bullet>>) {
    for mut pos in &mut query {
        pos.0 += BULLET_SPEED * time.delta_seconds();
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &XPosition, &Bullet)>,
    mut zombie_query: Query<(Entity, &XPosition, &mut Health), With<Zombie>>,
) {
    for (bullet_entity, bullet_pos, bullet) in &bullet_query {
        for (zombie_entity, zombie_pos, mut zombie_health) in &mut zombie_query {
            if (bullet_pos.0 - zombie_pos.0).abs() < 12.0 {
                zombie_health.hp -= bullet.damage;
                info!("Bullet hit Zombie! Zombie HP: {}", zombie_health.hp);

                commands.entity(bullet_entity).despawn_recursive();

                if zombie_health.hp <= 0.0 {
                    commands.entity(zombie_entity).despawn_recursive();
                    info!("Zombie died!");
                }
                break;
            }
        }
    }
}

fn zombie_attack(
    plant_query: Query<&XPosition, With<Plant>>,
    mut zombie_query: Query<(&XPosition, &mut Zombie)>,
) {
    if let Ok(plant_pos) = plant_query.get_single() {
        for (zombie_pos, mut zombie) in &mut zombie_query {
            if zombie.state == ZombieState::Walking && zombie_pos.0 <= plant_pos.0 {
                zombie.state = ZombieState::Eating;
                zombie.speed = 0.0;
                info!("Zombie reached Peashooter and started EATING!");
            }
        }
    }
}

fn zombie_eating(
    mut commands: Commands,
    mut plant_query: Query<(Entity, &mut Health), With<Plant>>,
    zombie_query: Query<&Zombie>,
) {
    if let Ok((plant_entity, mut plant_health)) = plant_query.get_single_mut() {
        let eating_count = zombie_query
            .iter()
            .filter(|z| z.state == ZombieState::Eating && z.eat_timer.just_finished())
            .count();

        if eating_count > 0 {
            let total_damage = eating_count as f32 * ZOMBIE_EAT_DAMAGE;
            plant_health.hp -= total_damage;
            info!("Peashooter is being eaten! Plant HP: {}", plant_health.hp);

            if plant_health.hp <= 0.0 {
                commands.entity(plant_entity).despawn_recursive();
                info!("Peashooter was eaten!");
            }
        }
    }
}

fn plant_death(
    plant_query: Query<&Plant>,
    mut zombie_query: Query<&mut Zombie>,
) {
    if plant_query.is_empty() {
        for mut zombie in &mut zombie_query {
            if zombie.state == ZombieState::Eating {
                zombie.state = ZombieState::Walking;
                zombie.speed = ZOMBIE_WALK_SPEED;
                info!("Lawn cleared. Zombie resumed walking!");
            }
        }
    }
}

fn sync_transforms(mut query: Query<(&XPosition, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        transform.translation.x = map_x_to_screen(pos.0);
    }
}

fn update_health_bars(
    mut fill_query: Query<(&Parent, &mut Sprite), With<HealthBarFill>>,
    parent_query: Query<&Parent>,
    health_query: Query<&Health>,
) {
    for (fill_parent, mut sprite) in &mut fill_query {
        if let Ok(main_parent) = parent_query.get(fill_parent.get()) {
            if let Ok(health) = health_query.get(main_parent.get()) {
                let ratio = (health.hp / health.max_hp).clamp(0.0, 1.0);
                sprite.custom_size = Some(Vec2::new(50.0 * ratio, 6.0));
            }
        }
    }
}

fn check_game_over(
    zombie_query: Query<&XPosition, With<Zombie>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for pos in &zombie_query {
        if pos.0 <= 10.0 {
            info!("A zombie reached the house! GAME OVER!");
            state.set(GameState::GameOver);
            break;
        }
    }
}

fn update_info_text(
    plant_query: Query<&Health, With<Plant>>,
    zombie_query: Query<(&Health, &XPosition), With<Zombie>>,
    mut text_query: Query<&mut Text, With<InfoText>>,
    spawn_timer: Res<ZombieSpawnTimer>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        let plant_status = if let Ok(health) = plant_query.get_single() {
            format!("HP: {:.1}/{:.1}", health.hp, health.max_hp)
        } else {
            "DECEASED".to_string()
        };

        let mut zombie_status = String::new();
        for (idx, (health, pos)) in zombie_query.iter().enumerate() {
            zombie_status.push_str(&format!(
                "\n  Zombie #{}: HP={:.1}, X={:.1}",
                idx + 1, health.hp, pos.0
            ));
        }
        if zombie_status.is_empty() {
            zombie_status = "\n  No zombies on the lawn.".to_string();
        }

        text.sections[0].value = format!(
            "Plants vs. Zombies - Bevy Animated Peashooter\n\
             Assembled from 19 parts in PeaShooter.reanim\n\n\
             Peashooter Status: {}\n\n\
             Key triggers:\n\
             [Press 1]: Trigger Idle Animation (frames 4-28)\n\
             [Press 2]: Trigger Head Idle Animation (frames 29-53)\n\
             [Press 3]: Trigger Shooting Animation (frames 54-78)\n\
             [Press 4]: Trigger Full Idle Animation (frames 79-103)\n\n\
             [Active Zombies]: {}\n\n\
             Next spawn in: {:.1}s",
            plant_status, zombie_status, spawn_timer.0.remaining_secs()
        );
    }
}

// Allows user to manually trigger different animation loops for testing
fn handle_keyboard_triggers(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut plant_query: Query<&mut Plant>,
) {
    for mut plant in &mut plant_query {
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            plant.anim_state = PlantAnimState::Idle;
            plant.head_frame = 79;
            plant.stem_frame = 79;
            info!("Manually triggered Idle Animation loop");
        }
        if keyboard_input.just_pressed(KeyCode::Digit2) {
            plant.anim_state = PlantAnimState::Shooting;
            plant.head_frame = 29;
            info!("Manually triggered Head Idle Animation (29-53)");
        }
        if keyboard_input.just_pressed(KeyCode::Digit3) {
            plant.anim_state = PlantAnimState::Shooting;
            plant.head_frame = 54;
            info!("Manually triggered Shooting Animation (54-78)");
        }
        if keyboard_input.just_pressed(KeyCode::Digit4) {
            plant.anim_state = PlantAnimState::Idle;
            plant.head_frame = 79;
            plant.stem_frame = 79;
            info!("Manually triggered Full Idle Animation (79-103)");
        }
    }
}
