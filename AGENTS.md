# bevy_playground — Agent Guide

## Commands
- `cargo build` — compile
- `cargo run` — run (dynamic linking enabled via `bevy` feature)
- No separate lint, typecheck, or test setup yet.

## Architecture
- Bevy 0.18.1, Rust edition 2024
- Plugin-based modules in `src/`:
  - `camera.rs` — `CameraPlugin`: orbit camera (`FollowCamera` marker), `CameraState` resource (yaw/pitch/distance)
  - `player.rs` — `PlayerPlugin`: `Player` component, WASD movement relative to camera yaw
  - `level.rs` — `LevelPlugin`: ground, reference cubes, lighting
  - `cursor.rs` — `CursorPlugin`: grabs/hides cursor on startup, Escape toggles
- `player_movement` reads `CameraState.yaw` for direction (not a separate angle resource)

## Bevy Gotchas
- `Transform` queries on different entity types must use `Without<T>` to avoid B0001 query conflict errors. Example: `Query<&mut Transform, (With<Camera3d>, Without<Player>)>`
- Bevy 0.18 API differs from older versions (e.g. `Transform::from_translation` not `Transform::from(Vec3)`)

## Dev Profile
- `opt-level = 1` for workspace, `opt-level = 3` for dependencies (needed for Bevy performance)
