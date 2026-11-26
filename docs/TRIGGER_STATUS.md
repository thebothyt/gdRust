# trigger status

what's actually been checked vs guessed. nothing here has been imported into a real GD 2.2 yet, so treat
"verified" as "matches G.js / gdlib source", not "play-tested".

## verified (object id + property keys match G.js or gdlib)

Color, Alpha, Toggle, Animate, Move (incl. lock/target/small step), Rotate, Scale, Follow, FollowPlayerY,
Pulse, Shake, Spawn (delay + remap), Stop, Random, AdvancedRandom, Sequence, Count, InstantCount, Pickup,
ItemEdit, ItemCompare, Time, TimeControl, Collision, CollisionBlock, Touch, OnDeath, Event, CameraZoom,
StaticCamera, CameraOffset, CameraRotate, CameraEdge, CameraMode, Song, Options, PlayerControl, TimeWarp,
Reverse, SpawnParticle, Gradient, ChangeMg, all orbs / pads / portals / smart blocks (D, J, S, H) / user coin.

## ids confirmed, keys partly confirmed

- Shader triggers: ids from the community id list. speed/strength keys (175/176) are from G.js, anything
  shader specific goes in `extra` as raw `(key, value)`.
- AdvancedFollow / EditAdvancedFollow / RetargetAdvancedFollow: ids confirmed, config is raw `(key, value)` pairs.
- CameraGuide: key names from gdlib, positions assumed to share the move keys.
- LinkVisible, Reset: target key only.

## ids confirmed, keys guessed

These place the right trigger, but the per-type settings may not land. Check them in the editor and fix
with `.set(key, value)` if needed.

- Area / EditArea / Enter triggers. shared area keys (length, mods, easings, relative) come from G.js,
  per-type values (move, rotate, scale, fade, tint) are guesses. AreaStop takes an effect id (key 51 per gdlib).
- ChangeBg, ChangeGround (reuse the mg key 533).
- Keyframe, KeyframeAnim.
- GameplayOffset, Arrow (direction only), Bpm, Sfx, EditSong.
- PersistentItem reset flag (493), TimeEvent multi flag (475), Spawn delay variance (556).

## not implemented

- `SmartBlock::f_block`: no id found anywhere, so it isn't there instead of being made up.
- `Collectible::new`: same story. `ItemDisplay` (the counter) is there, it's verified.
- `SpawnParticleTrigger` has no `count` arg, there's no such property on the trigger.

## differs from the original spec on purpose

- `PlayerControlTrigger::new(stop_jump, stop_move, stop_rot, stop_slide)`. the real trigger has no separate
  left/right toggles.
- `AreaStopTrigger::new(effect_id)`. it targets an effect id, not groups.
- `ArrowTrigger::new(direction_degrees)`. gravity mode isn't mapped.
- `BpmTrigger`, `ChangeBg/Ground`, decorations like `Deco::glow` / `Deco::chain` / `Deco::detail_border` use
  best-known ids, see source comments.

## file format

The `.gmd` wrapper and the level header `kA` keys are written from memory of the 2.2 format. If an import
fails or comes in with the wrong gamemode/speed, that's the first place to look (`level.rs`).
