# Kế hoạch triển khai Plants vs Zombies — Bevy / Rust

## Nguyên tắc kiến trúc chung

- **Plugin theo hệ thống**: mỗi mảng lớn (Animation, Grid, Plant, Zombie, Projectile, Sun, UI, Wave, Save) là một `Plugin` riêng trong Bevy — dễ bật/tắt, dễ test độc lập.
- **Component composition, không kế thừa**: một cây không phải "class Peashooter" mà là entity gồm nhiều component nhỏ, VD `Plant + Shooter + Health + GridPos`. Zombie thường = `Zombie + Health + Speed`; zombie có nón chỉ cần cộng thêm `ArmorHealth`.
- **Event-driven, tránh hệ thống gọi thẳng nhau**: dùng `ZombieDiedEvent`, `PlantEatenEvent`, `SunCollectedEvent`, `ProjectileHitEvent`... để các system độc lập lắng nghe. Việc này cực quan trọng khi sau này thêm Cherry Bomb (nổ diện rộng), Ice-shroom (làm chậm toàn map) — không phải sửa lại code cũ.
- **`GridPos` là chân lý, `Transform` chỉ là kết quả render**: logic (di chuyển, đặt cây, va chạm) luôn thao tác trên `GridPos`; có một system `sync_position` chạy cuối cùng để đồng bộ `GridPos → Transform`. Giúp debug dễ hơn nhiều so với sửa trực tiếp Transform.
- **Data-driven bằng RON/serde**: định nghĩa `PlantConfig`, `ZombieConfig` bằng file RON thay vì hardcode trong Rust. Thêm loại cây/zombie mới chỉ cần thêm config + tối đa 1-2 component đặc thù, không đụng vào core logic.
- **State machine tổng (`bevy_state`)**: `MainMenu → ChoosePlants → InGame → Paused → Win/Lose`. Set up từ sớm để tránh rối khi thêm màn chọn cây, pause, win screen.

---

## Component cốt lõi

```rust
// --- Components ---
struct Plant;
struct Zombie;
struct Bullet;

struct Health { current: u32, max: u32 }
struct Damage { value: u32 }
struct Speed { x: f32 }

// Vị trí logic — dùng thay Transform để tính toán
struct GridPos { row: u8, col: u8 }

// Animation
struct Animator {
    timer: Timer,
    frame: usize,
    state: AnimState, // Idle, Walk, Attack, Eat, Die
}

// Marker riêng từng loại (giai đoạn đầu, trước khi chuyển data-driven)
struct Peashooter;
struct Sunflower;
struct NormalZombie;
struct AttackCooldown { timer: Timer }
```

---

## Lộ trình triển khai (milestone chạy demo được ở mỗi bước)

### Giai đoạn 0 — Xương sống: Asset & Scene
1. **Asset Plugin & TextureAtlas**: gộp frame idle/walk/attack của từng entity qua `TextureAtlasLayout`. Nếu sprite đang rời file, dùng `TexturePacker` gộp thành 1 sheet trước để tăng FPS.
2. **Camera & Grid System**: camera 2D cố định; hệ tọa độ logic `row: 0..5, col: 0..9`; hàm `grid_to_world(row, col) -> Vec2`.
3. **Animation Plugin**: system `animate_sprite` dùng `Timer` chạy vòng lặp frame theo `AnimState`.

> ✅ Milestone: 1 zombie đứng trên màn hình chạy animation đi bộ lặp.

### Giai đoạn 1 — Vòng lặp gameplay lõi (làm trên **1 lane duy nhất** trước)
Đây là điểm mấu chốt: build đủ 1 hàng để có full vòng lặp "đặt cây → zombie tới → bắn → chết" chạy được, **trước khi** đầu tư UI/kinh tế sun. Tránh làm 5 hàng ngay từ đầu vì bug targeting/collision sẽ chồng chéo khó tách.

1. **Placement cơ bản**: click vào ô grid trống → spawn Peashooter (chưa cần trừ sun, có thể debug bằng phím tắt).
2. **Zombie Spawner (tạm thời, debug)**: spawn zombie bằng phím tắt hoặc timer đơn giản ở cột cuối cùng.
3. **Movement**: mỗi frame giảm `GridPos`/x-offset theo `Speed * delta_seconds`.
4. **Tầm nhìn & bắn**: Peashooter scan cùng row, tìm zombie gần nhất trong tầm → chuyển state `Shooting`, spawn `Bullet`.
5. **Va chạm & sát thương**: system check AABB/khoảng cách giữa Bullet và Zombie → trừ `Health`, bắn `ProjectileHitEvent`; Health ≤ 0 → bắn `ZombieDiedEvent` → despawn.
6. **Zombie ăn cây**: zombie đứng sát cây → dừng di chuyển, chuyển `AnimState::Eat`, trừ máu cây theo giây, bắn `PlantEatenEvent`.
7. **Game Over cơ bản**: zombie chạm cột 0 → trigger Lose.

> ✅ Milestone: chơi được 1 hàng — đặt Peashooter, zombie xuất hiện, bắn chết, hoặc bị ăn tới nhà.

### Giai đoạn 2 — Mở rộng ra 5 lane
Nhân rộng grid, spawner, targeting logic ra đủ 5 hàng. Vì logic đã validate ở 1 lane, bước này chủ yếu là generalize code (bỏ hardcode row).

### Giai đoạn 3 — Kinh tế Sun
1. **`Resource SunCount`**.
2. **Sun rơi tự nhiên**: timer random spawn entity `Sun` rơi từ trên xuống; click để thu → bắn `SunCollectedEvent`.
3. **`Sunflower`**: component `SunProducer` sinh sun theo interval.
4. **Ràng buộc đặt cây theo sun**: kiểm tra đủ sun + cooldown trước khi cho spawn (gắn vào logic placement ở Giai đoạn 1).

### Giai đoạn 4 — UI/HUD (`bevy_ui`)
- Seed packet bar: icon cây + overlay cooldown + mờ khi không đủ sun.
- Sun counter, thanh máu Plant/Zombie, progress bar sóng, nút xẻng (shovel) để đào cây.

### Giai đoạn 5 — Wave System & AppState
1. **`Resource WaveTimeline(Vec<WaveEvent { time, zombie_type, row }>)`** load từ file level data — thay cho spawn random ở Giai đoạn 1.
2. **State machine đầy đủ**: `MainMenu → LevelSelect → ChoosePlants → InGame → Paused → Win/Lose`.
3. **Win condition**: hết wave + không còn zombie sống.

### Giai đoạn 6 — Data-driven mở rộng nội dung
- Chuyển từ marker struct (`Peashooter`, `Sunflower`...) sang `PlantConfig`/`ZombieConfig` định nghĩa bằng RON, load qua `serde` + asset loader của Bevy.
- Thêm Wall-nut, Cherry Bomb, Snow Pea, Conehead/Buckethead... chỉ cần thêm config + component đặc thù (`Slow`, `AreaDamage`, `ArmorHealth`) — không viết lại hệ thống core.

### Giai đoạn 7 — Save/Progress
- Serialize qua `serde` (RON/JSON): level đã unlock, cây đã unlock, high score.
- Màn chọn cây trước level (giới hạn 6 slot theo bản gốc).

### Giai đoạn 8 — Polish
- Âm thanh (`bevy_kira_audio`), particle (`bevy_hanabi` nếu muốn hiệu ứng nổ/rơi sun).
- Screen shake khi Cherry Bomb nổ, lawnmower cứu nguy, biến thể level (bể bơi, ban đêm, mái nhà).
- Debug tooling: gắn `bevy_inspector_egui` để xem Health/GridPos runtime.
- Performance: nếu lag khi nhiều zombie, cân nhắc `bevy_ecs_tilemap` hoặc giới hạn tần suất check va chạm.

---

## Lời khuyên thực chiến

1. **Ưu tiên vòng lặp gameplay lõi trước UI đẹp** — 1 lane chạy được quan trọng hơn 5 lane có UI xịn nhưng chưa combat.
2. **Event trước, hardcode logic sau** — càng sớm dùng Event cho Died/Hit/Eaten, càng dễ thêm cơ chế đặc biệt sau này.
3. **Đừng chuyển data-driven quá sớm** — 2-3 loại cây/zombie đầu tiên cứ hardcode component cho nhanh, đến khi số loại tăng (~5+) mới đáng đầu tư RON config.
4. **Log/inspect liên tục** — `bevy_inspector_egui` giúp tiết kiệm rất nhiều thời gian debug Health/GridPos so với println.