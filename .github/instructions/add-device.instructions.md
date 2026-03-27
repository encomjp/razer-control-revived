---
applyTo: "razer_control_gui/data/devices/**"
description: "Use when adding, editing, or removing Razer device entries in laptops.json. Enforces multi-file coordination: udev rules, PID format, feature tags, and fan range conventions."
---
# Adding or Modifying Device Entries

When editing `laptops.json`, always coordinate changes across multiple files.

## Required steps for every new device

1. **Add JSON entry** to `razer_control_gui/data/devices/laptops.json`:
   ```json
   {
       "name": "Blade XX YYYY",
       "vid": "1532",
       "pid": "02XX",
       "features": ["logo", "boost", "bho"],
       "fan": [2200, 5000]
   }
   ```

2. **Add PID to udev rules** in `razer_control_gui/data/udev/99-hidraw-permissions.rules` — append the lowercase PID (without `0x` prefix) to the `ATTRS{idProduct}==` pipe-separated list. Keep the list sorted by PID value.

3. **Verify** with `cd razer_control_gui && cargo build --release` — the device list is loaded at runtime from the JSON file, so compilation confirms valid JSON but not hardware correctness.

## Field conventions

- `vid` — Always `"1532"` (Razer vendor ID)
- `pid` — 4-char uppercase hex string without `0x` prefix (e.g. `"02C7"`, not `"0x02c7"`)
- `features` — Subset of: `"fan"`, `"logo"`, `"boost"`, `"per_key_rgb"`, `"bho"`, `"creator_mode"`. Only include features confirmed for the specific model. When uncertain, start with `["logo"]` as the minimum.
- `fan` — `[min_rpm, max_rpm]`. Common ranges: older models `[3500, 5000]` or `[3500, 5300]`; 2023+ models typically `[2200, 5000]`
- `name` — Format: `"Blade {size} {year} {variant?}"` (e.g. `"Blade 16 2024"`, `"Blade 15 2019 Advanced"`)

## Udev PID format

In the udev rule, PIDs are **lowercase** without the `0x` prefix (e.g. `02c7`). This differs from `laptops.json` where PIDs are uppercase (`02C7`).
