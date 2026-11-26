# gdRust

Write Geometry Dash 2.2 levels in code. Flat API, fluent builders, exports a `.gmd` you can import.
Inspired by SPWN, G.js and Neditz.

```rust
use gd_rust::prelude::*;

fn main() {
    let mut level = Level::new("my level");
    level.settings.speed = Speed::Double;

    let mover = level.next_group();
    level.add(Block::default(5.0, 0.0).add_groups(&[mover]));
    level.add(MoveTrigger::new(mover, 4.0, 0.0, 1.0, Easing::EaseInOut).at(2.0, 8.0).touch_triggered(true));

    level.export_gmd("out.gmd").unwrap();
}
```

Coordinates are blocks (1.0 = one grid block = 30 gd units). Groups, colors, items and collision ids
come from `level.next_group()`, `next_color()`, `next_item()`, `next_block_id()`, so nothing collides.

## use it

**VS Code + CLI**

```bash
git clone https://github.com/thebothyt/gdRust && cd gdRust
cargo install --path crates/gdrust_cli
export GDRUST_PATH="$PWD/crates/gd_rust_core"   # until the core is on crates.io
gdrust build my_level.rs -o my_level.gmd
```

Scripts are normal Rust, so rust-analyzer autocomplete just works. Prebuilt release binaries aren't set up yet.

**Web playground**

`web/` is a static site that runs the compiler as WASM, no server. Scripts there are
[rhai](https://rhai.rs), which looks like Rust but isn't identical. Same names, same argument order:

```rust
let mover = next_group();
add(Block::default(5, 0).add_groups([mover]));
add(MoveTrigger::new(mover, 4, 0, 1, Easing::EaseInOut).at(2, 8).touch_triggered(true));
```

Differences from the Rust API: `let` instead of `let mut`, `add(...)` instead of `level.add(...)`,
settings through `set_mode(Mode::Ship)`, `set_speed(...)`, `set_name(...)`, ids are plain ints, and config
structs are maps like `#{ length: 300, relative: true }`.

Run it locally: `cd web && python3 -m http.server`, open `localhost:8000`.

**Deploy to Vercel**

1. import the repo, leave Framework Preset on "Other". `vercel.json` already points the output at `web/`.
2. no build command. `web/gdrust.wasm` is committed, Vercel's build image doesn't ship Rust.
3. after changing the core or wasm crate: `./scripts/build-web.sh`, commit the new `gdrust.wasm`.

## api

Objects: `Block`, `Spike`, `Sawblade`, `SmartBlock`, `Orb`, `Pad`, `Portal`, `UserCoin`, `ItemDisplay`,
`CollisionBlock`, `Deco`.

Modifiers on any object: `.at()`, `.add_groups()`, `.set_parent()`, `.with_color()`, `.with_detail_color()`,
`.with_hsv()`, `.with_z()`, `.with_transform()`, `.with_warp()`, `.set_editor_layer()`, `.set_flags()`,
`.spawn_triggered()`, `.multi_triggered()`, `.touch_triggered()`.

Triggers: color / pulse / alpha / toggle / animate / bg+ground+mg, move / rotate / scale / follow /
advanced follow / keyframes, area + enter, spawn / stop / reset / random / sequence, counters, timers,
collision / touch / death / events, camera, shaders, song / sfx / options / player control.
Full list is in `crates/gd_rust_core/src/triggers.rs`.

Use `.set(key, value)` on any object to write a raw property when something isn't wrapped yet.

## heads up

Not every trigger has been checked against the real game. Read [docs/TRIGGER_STATUS.md](docs/TRIGGER_STATUS.md)
before you trust a specific id or key, and import test levels into a throwaway save first.

## layout

```
crates/gd_rust_core   the library, no fs when built with --no-default-features (wasm safe)
crates/gdrust_cli     gdrust build level.rs
crates/gd_rust_wasm   rhai playground, plain C ABI, no wasm-bindgen
web/                  playground site
```
