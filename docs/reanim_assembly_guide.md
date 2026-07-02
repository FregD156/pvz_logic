# PopCap `.reanim` Animation Assembly & Rendering Guide for AI Agents

This document outlines the engineering specifications, mathematical formulas, and architectural patterns required to correctly parse, assemble, and render 2D multi-part animations from PopCap Games (e.g., *Plants vs. Zombies*) in a modern coordinate-based game engine (such as Bevy, Unity, or Godot).

---

## 1. File Structure Analysis

A PopCap `.reanim` file is a flat, XML-based timeline format describing individual tracks (body parts) and their keyframe properties over time.

### The XML Schema
- **`<fps>`**: Defines the playback speed (typically `12` FPS for *Plants vs. Zombies*).
- **`<track>`**: Represents a single body part layer (e.g., `backleaf`, `stalk_bottom`, `anim_face`).
  - **`<name>`**: The unique identifier of the track.
  - **`<t>`**: A keyframe (timeline step). The number of `<t>` tags in a track equals the total frames in the animation timeline.
    - **`<x>`**, **`<y>`**: Local translation offsets.
    - **`<sx>`**, **`<sy>`**: Scaling multipliers (default to `1.0`).
    - **`<kx>`**, **`<ky>`**: Skew angles in degrees (used to calculate rotation).
    - **`<f>`**: Frame visibility flag (`0` or greater = visible, `-1` = hidden/disabled).
    - **`<i>`**: The uppercase asset resource string (e.g., `IMAGE_REANIM_PEASHOOTER_HEAD`).

---

## 2. Step 1: Parsing & Keyframe Property Inheritance

The timeline is baked frame-by-frame, but the XML only declares attributes when they **change**. Properties must carry over (inherit) from the last defined keyframe:

1. Traverse each `<track>` sequentially.
2. Initialize default values for the track:
   - `x = 0.0`, `y = 0.0`
   - `sx = 1.0`, `sy = 1.0`
   - `kx = 0.0`, `ky = 0.0`
   - `f = -1` (default hidden)
   - `image = None`
3. For each `<t>` node in the track:
   - If an attribute (e.g., `<x>`) is declared, update the track's current value for that attribute.
   - If an attribute is **missing**, retain the value from the previous frame.
   - Resolve and push the final frame state into a flat list representing all frames (typically 104 frames).

---

## 3. Step 2: Coordinate Space & Transformation Matrix Mapping

PopCap's coordinate system originates from screen space, whereas modern game engines use Cartesian spaces.

### Coordinate Conversions
1. **Vertical Flip (Y-axis)**: Negate the Y coordinate because PopCap's Y increases downwards, whereas Cartesian Y increases upwards.
   $$\text{Translation}_y = -y$$
2. **Anchor / Registration Point**:
   > [!IMPORTANT]
   > PopCap coordinates specify the position of the **Top-Left** corner of the image, not the center.
   > You **must** set the sprite anchor/origin of all child parts to **Top-Left** (`bevy::sprite::Anchor::TopLeft`). If using center-anchored sprites, you will scramble the layout because image dimensions vary.
3. **Rotation (Skew to Angle)**:
   PopCap uses `kx` and `ky` to represent skew. For 2D sprite rotations, the rotation angle in radians can be derived from `kx` (degrees) around the Z-axis, negated:
   $$\theta = -\text{kx} \times \frac{\pi}{180.0}$$
   $$\text{Rotation} = \text{Quaternion::from\_rotation\_z}(\theta)$$

---

## 4. Step 3: Asset Resource Mapping

Resource strings declared in the XML (e.g., `IMAGE_REANIM_PEASHOOTER_HEAD`) must be resolved to their corresponding physical PNG files.

- Scan the asset folder case-insensitively.
- The default mapping follows: `IMAGE_REANIM_[PLANTNAME]_[PARTNAME]` $\rightarrow$ `[PlantName]_[partname].png`
- Example: `IMAGE_REANIM_PEASHOOTER_STALK_BOTTOM` maps to `PeaShooter_stalk_bottom.png`.

---

## 4. Step 4: Draw Order (Z-Indexing)

A `.reanim` file is structured such that background tracks are defined **first** (top of the file) and foreground tracks are defined **last** (bottom of the file).

To preserve the correct rendering stack:
- Assign a relative Z-coordinate to each child sprite based on its track index.
  $$z = \text{track\_index} \times 0.01$$
- This guarantees background parts (e.g. `backleaf` at index 0) render behind foreground parts (e.g. `anim_face` at index 20) without overlapping bugs.

---

## 5. Step 5: Multi-Track Animation Blending (Overlay System)

In PopCap animations, different parts of a character can play separate animations simultaneously (e.g., the stem loops `Idle` while the head plays `Shooting`).

### The Visibility Trap
- In `PeaShooter.reanim`, the head tracks are only visible in `HeadIdle` (29-53) and `Shooting` (54-78) ranges. They are hidden (`f = -1`) in the `Idle` (4-28) range.
- Conversely, stem and leaf tracks are hidden in `HeadIdle` and `Shooting` ranges.
- Playing a single timeline index causes parts of the plant to disappear.

### The Blending Rule
To render a complete animated character, run **two parallel frame counters**:
1. **`stem_frame`**: Always loops the base idle animation (e.g. `FullIdle` range `79..=103`).
2. **`head_frame`**: Plays the active state (e.g. `Shooting` range `54..=78` or `HeadIdle` range `29..=53`).

Apply transformations dynamically using **Visibility Fallback**:
```rust
// For each track / body part:
let head_data = track.frames[head_frame];
let stem_data = track.frames[stem_frame];

let target_frame_data = if head_data.visible {
    head_data // Use current active head frame if visible
} else {
    stem_data // Fallback to looping stem frame if hidden in head frame
};
```
This ensures the stem continues its fluid idle motion while the head overlays its shooting or nháy mắt animations correctly.

---

## 6. Reference Implementation (Rust / Bevy 0.14)

```rust
fn animate_plant(
    reanim_data: Res<ReanimData>,
    reanim_textures: Res<ReanimTextures>,
    mut plant_query: Query<(Entity, &mut Plant)>,
    parent_query: Query<&Children>,
    mut part_query: Query<(&ReanimPart, &mut Sprite, &mut Transform, &mut Visibility, &mut Handle<Image>)>,
) {
    for (plant_entity, mut plant) in &mut plant_query {
        // Ticks frame counters at 12 FPS
        if plant.anim_timer.just_finished() {
            // Loop stem frame
            plant.stem_frame += 1;
            if plant.stem_frame > 103 {
                plant.stem_frame = 79;
            }

            // Update head frame based on state
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

        // Apply transformations to child tracks
        if let Ok(children) = parent_query.get(plant_entity) {
            for &child in children {
                if let Ok((part, mut sprite, mut transform, mut visibility, mut texture)) = part_query.get_mut(child) {
                    let track = &reanim_data.tracks[part.track_index];
                    
                    let head_data = &track.frames[plant.head_frame];
                    let stem_data = &track.frames[plant.stem_frame];

                    // Track Blending Visibility Fallback
                    let frame_data = if head_data.visible { head_data } else { stem_data };

                    if frame_data.visible {
                        *visibility = Visibility::Inherited;
                        
                        // Load sprite frame texture
                        if let Some(ref img_res) = frame_data.image {
                            if let Some(handle) = reanim_textures.handles.get(img_res) {
                                *texture = handle.clone();
                                sprite.custom_size = None;
                            }
                        }
                        
                        // Translate Top-Left Anchor
                        transform.translation.x = frame_data.x;
                        transform.translation.y = -frame_data.y; // Negate Y
                        transform.translation.z = part.track_index as f32 * 0.01; // Z-ordering
                        
                        // Scale
                        transform.scale.x = frame_data.sx;
                        transform.scale.y = frame_data.sy;
                        
                        // Rotate (convert degrees kx to Z-axis radians)
                        transform.rotation = Quat::from_rotation_z(-frame_data.kx.to_radians());
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}
```
