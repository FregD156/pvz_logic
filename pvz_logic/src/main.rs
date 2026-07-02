use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use std::fs;

// 2D Grid Configuration (checkered lawn)
const GRID_ROWS: usize = 5;
const GRID_COLS: usize = 9;
const CELL_WIDTH: f32 = 80.0;
const CELL_HEIGHT: f32 = 90.0;
const LAWN_ORIGIN_X: f32 = -360.0; // Left edge of the lawn
const LAWN_ORIGIN_Y: f32 = 220.0;  // Top edge of the lawn

// Coordinate mapping helper
fn get_cell_center(row: usize, col: usize) -> Vec2 {
    Vec2::new(
        LAWN_ORIGIN_X + col as f32 * CELL_WIDTH + CELL_WIDTH / 2.0,
        LAWN_ORIGIN_Y - row as f32 * CELL_HEIGHT - CELL_HEIGHT / 2.0,
    )
}

// Gameplay Constants
const ZOMBIE_WALK_SPEED: f32 = 25.0;
const BULLET_SPEED: f32 = 300.0;
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
struct GridPosition {
    row: usize,
    col: usize,
}

#[derive(Component)]
struct Health {
    hp: f32,
    max_hp: f32,
}

#[derive(Component)]
struct Plant {
    anim_name: String, // "peashooter", "repeater", or "cattail"
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
    row: usize,
    damage: f32,
}

#[derive(Component)]
struct Zombie {
    row: usize,
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
        "IMAGE_REANIM_ANIM_SPROUT" => Some("PvZ_Assets/reanim/anim_sprout.png"),

        // Cattail parts
        "IMAGE_REANIM_CATTAIL_TAIL2_OVERLAY" => Some("PvZ_Assets/reanim/Cattail_tail2_overlay.png"),
        "IMAGE_REANIM_CATTAIL_SPIKE" => Some("PvZ_Assets/reanim/Cattail_spike.png"),
        "IMAGE_REANIM_CATTAIL_TAIL2" => Some("PvZ_Assets/reanim/Cattail_tail2.png"),
        "IMAGE_REANIM_CATTAIL_PAW1" => Some("PvZ_Assets/reanim/Cattail_paw1.png"),
        "IMAGE_REANIM_CATTAIL_PAW3" => Some("PvZ_Assets/reanim/Cattail_paw3.png"),
        "IMAGE_REANIM_CATTAIL_PAW2" => Some("PvZ_Assets/reanim/Cattail_paw2.png"),
        "IMAGE_REANIM_CATTAIL_BLINK2" => Some("PvZ_Assets/reanim/Cattail_blink2.png"),
        "IMAGE_REANIM_CATTAIL_BLINK1" => Some("PvZ_Assets/reanim/Cattail_blink1.png"),
        "IMAGE_REANIM_CATTAIL_EYEBROW2" => Some("PvZ_Assets/reanim/Cattail_eyebrow2.png"),
        "IMAGE_REANIM_CATTAIL_TAIL" => Some("PvZ_Assets/reanim/Cattail_tail.png"),
        "IMAGE_REANIM_CATTAIL_HEAD" => Some("PvZ_Assets/reanim/Cattail_head.png"),
        "IMAGE_REANIM_CATTAIL_EYEBROW1" => Some("PvZ_Assets/reanim/Cattail_eyebrow1.png"),
        "IMAGE_REANIM_CATTAIL_HAT" => Some("PvZ_Assets/reanim/Cattail_hat.png"),
        "IMAGE_REANIM_CATTAIL_BLINK" => Some("PvZ_Assets/reanim/Cattail_blink.png"),
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
    let single_xml = fs::read_to_string("assets/PvZ_Assets/reanim/PeaShooterSingle.reanim")
        .expect("Failed to read PeaShooterSingle.reanim file.");
    let single_tracks = parse_reanim(&single_xml);

    // 2. Read and parse Repeater (PeaShooter.reanim)
    let repeater_xml = fs::read_to_string("assets/PvZ_Assets/reanim/PeaShooter.reanim")
        .expect("Failed to read PeaShooter.reanim file.");
    let repeater_tracks = parse_reanim(&repeater_xml);

    // 3. Read and parse Cattail
    let cattail_xml = fs::read_to_string("assets/PvZ_Assets/reanim/Cattail.reanim")
        .expect("Failed to read Cattail.reanim file.");
    let cattail_tracks = parse_reanim(&cattail_xml);

    let mut library = HashMap::new();
    library.insert("peashooter".to_string(), single_tracks);
    library.insert("repeater".to_string(), repeater_tracks);
    library.insert("cattail".to_string(), cattail_tracks);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Plants vs. Zombies - 5-Lane Checkered Lawn Showcase".into(),
                resolution: (1000.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(ReanimLibrary { animations: library })
        .insert_resource(ZombieSpawnTimer(Timer::from_seconds(6.0, TimerMode::Repeating)))
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
        ).run_if(in_state(GameState::Playing)))
        .add_systems(Update, check_game_over.run_if(in_state(GameState::Playing)))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    reanim_library: Res<ReanimLibrary>,
) {
    // 2D Camera
    commands.spawn(Camera2dBundle::default());

    // Render checkered lawn (Odd lanes: light green, Even lanes: dark green)
    for r in 0..GRID_ROWS {
        for c in 0..GRID_COLS {
            let center = get_cell_center(r, c);
            let color = if (r + c) % 2 == 0 {
                Color::srgb(0.38, 0.65, 0.16) // Even cells: Dark green
            } else {
                Color::srgb(0.45, 0.72, 0.22) // Odd cells: Light green
            };

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(CELL_WIDTH, CELL_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(center.x, center.y, 0.0),
                ..default()
            });
        }
    }

    // Load all the required textures for all plants
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

    // Column 1: Peashooters (Single) on Rows 0, 2, 4
    spawn_plant(&mut commands, "peashooter", 0, 1, &reanim_library);
    spawn_plant(&mut commands, "peashooter", 2, 1, &reanim_library);
    spawn_plant(&mut commands, "peashooter", 4, 1, &reanim_library);

    // Column 2: Repeaters (Double) on Rows 1, 3
    spawn_plant(&mut commands, "repeater", 1, 2, &reanim_library);
    spawn_plant(&mut commands, "repeater", 3, 2, &reanim_library);

    // Column 3: Cattail on Row 2
    spawn_plant(&mut commands, "cattail", 2, 2, &reanim_library);

    // On-screen UI instructions
    commands.spawn((
        TextBundle::from_section(
            "Plants vs. Zombies - 5-Lane Checkered Lawn\n\
             =======================================\n\n\
             Pre-planted layout:\n\
             - Col 1: Peashooters (bắn 1 đạn)\n\
             - Col 2: Repeaters (bắn 2 đạn)\n\
             - Col 3: Cattails (bắn gai đuôi mèo)\n\n\
             Zombies spawn randomly in different lanes!\n\
             Bullets and collision checks are lane-based.",
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

fn spawn_plant(
    commands: &mut Commands,
    anim_name: &str,
    row: usize,
    col: usize,
    reanim_library: &ReanimLibrary,
) {
    let center = get_cell_center(row, col);
    
    // Choose start frame based on type
    let start_frame = if anim_name == "cattail" { 5 } else { 79 };

    // Parent Plant Entity
    // Shifts coordinates slightly left/up so anchor matches cell center
    let offset_x = if anim_name == "cattail" { -25.0 } else { -25.0 };
    let offset_y = if anim_name == "cattail" { 20.0 } else { 45.0 };

    let plant_entity = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(center.x + offset_x, center.y - offset_y, 2.0).with_scale(Vec3::splat(0.85)),
            ..default()
        },
        GridPosition { row, col },
        Health {
            hp: PLANT_MAX_HP,
            max_hp: PLANT_MAX_HP,
        },
        Plant {
            anim_name: anim_name.to_string(),
            shoot_timer: Timer::from_seconds(PLANT_SHOOT_COOLDOWN, TimerMode::Repeating),
            anim_state: PlantAnimState::Idle,
            head_frame: start_frame,
            stem_frame: start_frame,
            anim_timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    )).id();

    // Spawn body parts as children
    let tracks = reanim_library.animations.get(anim_name).unwrap();
    for (idx, track) in tracks.iter().enumerate() {
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
        )).set_parent(plant_entity);
    }

    spawn_health_bar(commands, plant_entity, 75.0);
}

fn spawn_zombie_at(commands: &mut Commands, row: usize) {
    // Spawn at right edge of row
    let center = get_cell_center(row, 8);
    let spawn_x = center.x + 100.0;

    let zombie_entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.3, 0.6),
                custom_size: Some(Vec2::new(45.0, 80.0)),
                ..default()
            },
            transform: Transform::from_xyz(spawn_x, center.y + 10.0, 2.0),
            ..default()
        },
        Zombie {
            row,
            speed: ZOMBIE_WALK_SPEED,
            state: ZombieState::Walking,
            eat_timer: Timer::from_seconds(ZOMBIE_EAT_COOLDOWN, TimerMode::Repeating),
        },
        Health {
            hp: ZOMBIE_MAX_HP,
            max_hp: ZOMBIE_MAX_HP,
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
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..GRID_ROWS);
        spawn_zombie_at(&mut commands, row);
        info!("A new Zombie spawned in Lane/Row {}", row);
    }
}

fn zombie_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Zombie)>) {
    for (mut transform, zombie) in &mut query {
        if zombie.state == ZombieState::Walking {
            transform.translation.x -= zombie.speed * time.delta_seconds();
        }
    }
}

// Plant shooting logic (lane-based)
fn plant_shooting(
    mut commands: Commands,
    mut plant_query: Query<(&Transform, &GridPosition, &mut Plant)>,
    zombie_query: Query<(&Transform, &Zombie)>,
) {
    for (plant_transform, grid_pos, mut plant) in &mut plant_query {
        // Find if there is any zombie in the same row ahead of the plant
        let zombie_in_range = zombie_query.iter().any(|(z_trans, zombie)| {
            zombie.row == grid_pos.row && z_trans.translation.x > plant_transform.translation.x
        });

        if zombie_in_range {
            if plant.shoot_timer.finished() {
                let start_x = plant_transform.translation.x + 20.0;
                let start_y = plant_transform.translation.y + 15.0;

                // Spawn bullets based on plant type
                match plant.anim_name.as_str() {
                    "repeater" => {
                        // Repeater shoots 2 bullets close to each other
                        for offset in &[0.0, -18.0] {
                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::srgb(0.9, 0.9, 0.1),
                                        custom_size: Some(Vec2::new(14.0, 14.0)),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(start_x + offset, start_y, 3.0),
                                    ..default()
                                },
                                Bullet {
                                    row: grid_pos.row,
                                    damage: BULLET_DAMAGE,
                                },
                            ));
                        }
                    }
                    _ => {
                        // Peashooter and Cattail shoot 1 bullet
                        let color = if plant.anim_name == "cattail" {
                            Color::srgb(1.0, 0.4, 0.6) // Cattail shoots pink spikes
                        } else {
                            Color::srgb(0.9, 0.9, 0.1) // Peashooter shoots yellow peas
                        };

                        commands.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color,
                                    custom_size: Some(Vec2::new(14.0, 14.0)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(start_x, start_y, 3.0),
                                ..default()
                            },
                            Bullet {
                                row: grid_pos.row,
                                damage: BULLET_DAMAGE,
                            },
                        ));
                    }
                }

                // Trigger Shooting animation
                plant.anim_state = PlantAnimState::Shooting;
                plant.head_frame = if plant.anim_name == "cattail" { 24 } else { 54 };

                plant.shoot_timer.reset();
            }
        } else {
            plant.shoot_timer.reset();
        }
    }
}

// Animation system for all plants
fn animate_plant(
    reanim_library: Res<ReanimLibrary>,
    reanim_textures: Res<ReanimTextures>,
    mut plant_query: Query<(Entity, &mut Plant)>,
    parent_query: Query<&Children>,
    mut part_query: Query<(&ReanimPart, &mut Sprite, &mut Transform, &mut Visibility, &mut Handle<Image>)>,
) {
    for (plant_entity, mut plant) in &mut plant_query {
        if plant.anim_timer.just_finished() {
            if plant.anim_name == "cattail" {
                plant.stem_frame += 1;
                if plant.stem_frame > 23 {
                    plant.stem_frame = 5;
                }

                match plant.anim_state {
                    PlantAnimState::Idle => {
                        plant.head_frame = plant.stem_frame;
                    }
                    PlantAnimState::Shooting => {
                        plant.head_frame += 1;
                        if plant.head_frame > 39 || plant.head_frame < 24 {
                            plant.anim_state = PlantAnimState::Idle;
                            plant.head_frame = plant.stem_frame;
                        }
                    }
                }
            } else {
                plant.stem_frame += 1;
                if plant.stem_frame > 103 {
                    plant.stem_frame = 79;
                }

                match plant.anim_state {
                    PlantAnimState::Idle => {
                        plant.head_frame = plant.stem_frame;
                    }
                    PlantAnimState::Shooting => {
                        plant.head_frame += 1;
                        if plant.head_frame > 78 || plant.head_frame < 54 {
                            plant.anim_state = PlantAnimState::Idle;
                            plant.head_frame = plant.stem_frame;
                        }
                    }
                }
            }
        }

        // Apply transformations
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
                                
                                transform.translation.x = frame_data.x * 0.8;
                                transform.translation.y = -frame_data.y * 0.8;
                                transform.translation.z = part.track_index as f32 * 0.01;
                                
                                transform.scale.x = frame_data.sx * 0.8;
                                transform.scale.y = frame_data.sy * 0.8;
                                
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

fn bullet_movement(time: Res<Time>, mut query: Query<&mut Transform, With<Bullet>>) {
    for mut transform in &mut query {
        transform.translation.x += BULLET_SPEED * time.delta_seconds();
    }
}

// Lane-based bullet collision
fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut zombie_query: Query<(Entity, &Transform, &mut Health, &Zombie)>,
) {
    for (bullet_entity, bullet_trans, bullet) in &bullet_query {
        for (zombie_entity, zombie_trans, mut zombie_health, zombie) in &mut zombie_query {
            // Must be in the same row
            if bullet.row == zombie.row {
                let dist = (bullet_trans.translation.x - zombie_trans.translation.x).abs();
                if dist < 15.0 {
                    zombie_health.hp -= bullet.damage;
                    info!("Bullet hit Zombie in Row {}! Zombie HP: {}", zombie.row, zombie_health.hp);

                    commands.entity(bullet_entity).despawn_recursive();

                    if zombie_health.hp <= 0.0 {
                        commands.entity(zombie_entity).despawn_recursive();
                        info!("Zombie died in Row {}!", zombie.row);
                    }
                    break;
                }
            }
        }
    }
}

// Lane-based zombie eating attack
fn zombie_attack(
    plant_query: Query<(&Transform, &GridPosition)>,
    mut zombie_query: Query<(&Transform, &mut Zombie)>,
) {
    for (plant_trans, grid_pos) in &plant_query {
        for (zombie_trans, mut zombie) in &mut zombie_query {
            // Must be in the same row and zombie is close to the plant
            if zombie.row == grid_pos.row && zombie.state == ZombieState::Walking {
                let dist = zombie_trans.translation.x - plant_trans.translation.x;
                // If zombie is directly on or slightly to the right of the plant
                if dist <= 12.0 && dist > -15.0 {
                    zombie.state = ZombieState::Eating;
                    zombie.speed = 0.0;
                    info!("Zombie in Row {} reached Plant and started eating!", zombie.row);
                }
            }
        }
    }
}

// Lane-based zombie eating damage
fn zombie_eating(
    mut commands: Commands,
    mut plant_query: Query<(Entity, &Transform, &GridPosition, &mut Health), With<Plant>>,
    zombie_query: Query<&Zombie>,
) {
    for (plant_entity, _, grid_pos, mut plant_health) in &mut plant_query {
        let eating_count = zombie_query
            .iter()
            .filter(|z| {
                z.row == grid_pos.row
                    && z.state == ZombieState::Eating
                    && z.eat_timer.just_finished()
            })
            .count();

        if eating_count > 0 {
            let total_damage = eating_count as f32 * ZOMBIE_EAT_DAMAGE;
            plant_health.hp -= total_damage;
            info!(
                "Plant in Row {}, Col {} is being eaten! HP: {}",
                grid_pos.row, grid_pos.col, plant_health.hp
            );

            if plant_health.hp <= 0.0 {
                commands.entity(plant_entity).despawn_recursive();
                info!("Plant in Row {}, Col {} was eaten!", grid_pos.row, grid_pos.col);
            }
        }
    }
}

// Release eating zombies if their plant died
fn plant_death(
    plant_query: Query<&GridPosition, With<Plant>>,
    mut zombie_query: Query<&mut Zombie>,
) {
    for mut zombie in &mut zombie_query {
        if zombie.state == ZombieState::Eating {
            // Check if any plant still exists in the same row at the column the zombie is eating
            let plant_exists = plant_query.iter().any(|gp| gp.row == zombie.row);
            if !plant_exists {
                zombie.state = ZombieState::Walking;
                zombie.speed = ZOMBIE_WALK_SPEED;
                info!("Lawn row {} cleared. Zombie resumed walking!", zombie.row);
            }
        }
    }
}

fn sync_transforms() {
    // Spatial positioning is fully mapped at spawn/movement, no separate sync needed
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

// Check if any zombie reached the house (X < -440.0)
fn check_game_over(
    zombie_query: Query<&Transform, With<Zombie>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for trans in &zombie_query {
        if trans.translation.x <= -420.0 {
            info!("A Zombie crossed the lawn and reached your house! GAME OVER!");
            state.set(GameState::GameOver);
            break;
        }
    }
}

fn update_info_text(
    plant_query: Query<(&Health, &GridPosition), With<Plant>>,
    zombie_query: Query<(&Health, &Transform, &Zombie)>,
    mut text_query: Query<&mut Text, With<InfoText>>,
    spawn_timer: Res<ZombieSpawnTimer>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        let plants_status = format!("Plants Alive: {}", plant_query.iter().count());
        let mut zombies_status = String::new();
        
        for (idx, (health, trans, zombie)) in zombie_query.iter().enumerate() {
            zombies_status.push_str(&format!(
                "\n  Zombie #{}: Row={}, HP={:.1}, X={:.1}",
                idx + 1, zombie.row, health.hp, trans.translation.x
            ));
        }
        if zombies_status.is_empty() {
            zombies_status = "\n  No zombies on the lawn.".to_string();
        }

        text.sections[0].value = format!(
            "Plants vs. Zombies - 5-Lane Checkered Lawn\n\
             =======================================\n\n\
             Defenses Status: {}\n\n\
             Zombies on Lawn: {}\n\n\
             Next spawn in: {:.1}s",
            plants_status, zombies_status, spawn_timer.0.remaining_secs()
        );
    }
}
