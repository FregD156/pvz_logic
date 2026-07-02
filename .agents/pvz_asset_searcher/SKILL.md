---
name: pvz-asset-searcher
description: Skill to search for asset files, textures, configurations, and sounds in the decrypted PvZ_Assets/ folder.
---

# PvZ Asset Searcher Skill

This skill provides resources and scripts to search for, inspect, and map asset files from the decrypted **Plants vs. Zombies** assets directory located at `/Users/fregd/Documents/code/playground/PvZ/PvZ_Assets`.

---

## 1. Asset Directory Structure

The `PvZ_Assets/` directory is organized as follows:
- **`images/`**: Core static game UI sprites, projectile textures (e.g., `ProjectilePea.png`), and generic sprites.
- **`reanim/`**: Multi-part PNG sprites and `.reanim` timeline configuration files (e.g., `PeaShooter.reanim`, `Cattail.reanim`).
- **`particles/`**: XML configuration files and texture Atlases for particle effects (e.g., `PeaSplat.xml`).
- **`properties/`**: XML configuration manifests (e.g., `resources.xml`, `default.xml`).
- **`sounds/`**: Sound effect files in `.ogg` or `.wav` format.

---

## 2. Using the Asset Searcher Script

We provide a helper python script `scripts/find_asset.py` to search for files inside `PvZ_Assets/` case-insensitively and match resource references.

### How to use:
Run the script using python from any workspace:
```bash
python3 /Users/fregd/.gemini/config/skills/pvz_asset_searcher/scripts/find_asset.py "<search_pattern>"
```

### Examples:
1. **Find by name**:
   ```bash
   python3 /Users/fregd/.gemini/config/skills/pvz_asset_searcher/scripts/find_asset.py "repeater"
   ```
2. **Find all reanim files**:
   ```bash
   python3 /Users/fregd/.gemini/config/skills/pvz_asset_searcher/scripts/find_asset.py ".reanim"
   ```
3. **Find images matching a plant**:
   ```bash
   python3 /Users/fregd/.gemini/config/skills/pvz_asset_searcher/scripts/find_asset.py "Cattail"
   ```
