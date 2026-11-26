use crate::obj::*;

// every effect trigger needs 36 or gd treats it like a plain object
fn t(id: u16) -> Obj { Obj::raw(id).set(36, 1) }
fn e(x: Easing) -> i32 { x as i32 }
// gd stores "units" where 30 = 1 block, everything here takes blocks
fn u(blocks: f32) -> f32 { blocks * 30.0 }

#[derive(Clone, Copy)] pub enum PulseMode { Color, Group }
#[derive(Clone, Copy)] pub enum StopMode { Stop, Pause, Resume }
#[derive(Clone, Copy)] pub enum SeqMode { Default, Loop, Snap }
#[derive(Clone, Copy)] pub enum Compare { Equal, Larger, Smaller }
#[derive(Clone, Copy)] pub enum MathOp { Set, Add, Sub, Mul, Div }
#[derive(Clone, Copy)] pub enum RoundMode { None, Round, Floor, Ceil }
#[derive(Clone, Copy)] pub enum ToggleMode { Normal, On, Off }
#[derive(Clone, Copy)] pub enum DualMode { Normal, Dual }
#[derive(Clone, Copy)] pub enum Edge { Left = 1, Right, Up, Down }
#[derive(Clone, Copy)] pub enum ItemType { Item = 1, Timer, Points, Time }
#[derive(Clone, Copy)] pub enum TransitionMode { Both, Enter, Exit }

#[derive(Clone, Copy)]
pub enum ShaderType {
    Layers, ShockWave, ShockLine, Glitch, Chromatic, ChromaticGlitch, Pixelate, LensCircle,
    RadialBlur, Blur, Bulge, Pinch, Grayscale, Sepia, Invert, HueShift, EditColor, SplitScreen,
}
// the old Shockwave/ShockLine names in the spec map straight onto these
pub use ShaderType::{ShockLine, ShockWave};

#[derive(Clone, Copy)]
pub enum OptionFlag {
    StreakAdditive, HideGround, HideMg, HideP1, HideP2, DisableControlsP1,
    DisableControlsP2, UnlinkDualGravity, HideAttempts, AudioOnDeath, NoDeathSfx,
}

// ---- builder methods that only make sense on specific triggers ----
impl Obj {
    // spawn trigger remaps, pairs are (old, new)
    pub fn remap(self, pairs: &[(u16, u16)]) -> Obj {
        let s = pairs.iter().map(|(a, b)| format!("{a}.{b}")).collect::<Vec<_>>().join(".");
        self.set(442, s)
    }
    pub fn easing_rate(self, rate: f32) -> Obj { self.set(85, rate) }
    pub fn silent(self, v: bool) -> Obj { self.set_bool(544, v) }
    pub fn rgb(self, r: u8, g: u8, b: u8) -> Obj { self.set(7, r).set(8, g).set(9, b) }
    pub fn p1(self, v: bool) -> Obj { self.set_bool(138, v) }
    pub fn p2(self, v: bool) -> Obj { self.set_bool(200, v) }
    pub fn channel(self, ch: u8) -> Obj { self.set(432, ch) }

    // move trigger
    pub fn lock_player(self, x: bool, y: bool) -> Obj { self.set_bool(58, x).set_bool(59, y) }
    pub fn lock_camera(self, x: bool, y: bool) -> Obj { self.set_bool(141, x).set_bool(142, y) }
    pub fn target_mode(self, target: Group, center: Group, dynamic: bool) -> Obj {
        self.set(100, 1).set(71, target.0).set(395, center.0).set_bool(397, dynamic)
    }
    pub fn small_step(self, v: bool) -> Obj { self.set_bool(393, v) }

    // pulse trigger
    pub fn pulse_hsv(self, h: i32, s: f32, b: f32, s_chk: bool, b_chk: bool) -> Obj {
        self.set(48, 1).set(49, format!("{h}a{s}a{b}a{}a{}", s_chk as u8, b_chk as u8))
    }

    // item triggers
    pub fn item_types(self, a: ItemType, b: ItemType, target: ItemType) -> Obj {
        self.set(476, a as u8).set(477, b as u8).set(478, target as u8)
    }
}

// ---- color, visual, environment ----

pub struct ColorTrigger;
impl ColorTrigger {
    pub fn new(target: Color, rgba: (u8, u8, u8, f32), duration: f32, blending: bool) -> Obj {
        t(899).set(23, target.0).rgb(rgba.0, rgba.1, rgba.2)
            .set(35, rgba.3).set(10, duration).set_bool(17, blending)
    }
}

pub struct PulseTrigger;
impl PulseTrigger {
    pub fn new(target: u16, mode: PulseMode, fade_in: f32, hold: f32, fade_out: f32) -> Obj {
        let kind = match mode { PulseMode::Color => 0, PulseMode::Group => 1 };
        t(1006).set(51, target).set(52, kind).set(45, fade_in).set(46, hold).set(47, fade_out)
    }
}

pub struct AlphaTrigger;
impl AlphaTrigger {
    pub fn new(target: Group, opacity: f32, duration: f32) -> Obj {
        t(1007).set(51, target.0).set(35, opacity).set(10, duration)
    }
}

pub struct ToggleTrigger;
impl ToggleTrigger {
    pub fn new(target: Group, activate: bool) -> Obj { t(1049).set(51, target.0).set_bool(56, activate) }
}

pub struct AnimateTrigger;
impl AnimateTrigger {
    pub fn new(target: Group, animation_id: i32) -> Obj { t(1585).set(51, target.0).set(76, animation_id) }
}

// bg/ground/mg id key is verified for mg only, bg and ground reuse it
pub struct ChangeBgTrigger;
impl ChangeBgTrigger {
    pub fn new(bg_id: u16, duration: f32) -> Obj { t(3029).set(533, bg_id).set(10, duration) }
}
pub struct ChangeGroundTrigger;
impl ChangeGroundTrigger {
    pub fn new(g_id: u16, duration: f32) -> Obj { t(3030).set(533, g_id).set(10, duration) }
}
pub struct ChangeMgTrigger;
impl ChangeMgTrigger {
    pub fn new(mg_id: u16, duration: f32) -> Obj { t(3031).set(533, mg_id).set(10, duration) }
}

#[derive(Clone)]
pub struct GradientCfg {
    pub bl: Group, pub br: Group, pub tl: Group, pub tr: Group,
    pub color1: Color, pub color2: Color,
    pub vertex_mode: bool, pub blending: bool, pub layer: u8,
}
pub struct GradientTrigger;
impl GradientTrigger {
    pub fn new(id: u16, c: GradientCfg) -> Obj {
        t(2903).set(209, id).set(203, c.bl.0).set(204, c.br.0).set(205, c.tl.0).set(206, c.tr.0)
            .set(21, c.color1.0).set(22, c.color2.0).set_bool(207, c.vertex_mode)
            .set_bool(174, c.blending).set(202, c.layer)
    }
}

pub struct SpawnParticleTrigger;
impl SpawnParticleTrigger {
    // offset from the position group, in blocks
    pub fn new(particle_group: Group, pos_group: Group, offset: GridPos, scale: f32, angle: f32) -> Obj {
        t(3608).set(51, particle_group.0).set(71, pos_group.0)
            .set(547, u(offset.0)).set(548, u(offset.1)).set(554, scale).set(552, angle)
    }
}

pub struct LinkVisibleTrigger;
impl LinkVisibleTrigger {
    pub fn new(target: Group) -> Obj { t(3662).set(51, target.0) }
}

// ---- movement, follow, keyframes ----

pub struct MoveTrigger;
impl MoveTrigger {
    pub fn new(target: Group, dx: f32, dy: f32, duration: f32, easing: Easing) -> Obj {
        t(901).set(51, target.0).set(28, u(dx)).set(29, u(dy)).set(10, duration).set(30, e(easing))
    }
}

pub struct RotateTrigger;
impl RotateTrigger {
    pub fn new(target: Group, center: Group, degrees: f32, times_360: i32, duration: f32, easing: Easing) -> Obj {
        t(1346).set(51, target.0).set(71, center.0).set(68, degrees).set(69, times_360)
            .set(10, duration).set(30, e(easing))
    }
}

pub struct ScaleTrigger;
impl ScaleTrigger {
    pub fn new(target: Group, center: Group, sx: f32, sy: f32, duration: f32, easing: Easing) -> Obj {
        t(2067).set(51, target.0).set(71, center.0).set(150, sx).set(151, sy)
            .set(10, duration).set(30, e(easing))
    }
}

pub struct FollowTrigger;
impl FollowTrigger {
    pub fn new(target: Group, follow: Group, x_mod: f32, y_mod: f32, duration: f32) -> Obj {
        t(1347).set(51, target.0).set(71, follow.0).set(72, x_mod).set(73, y_mod).set(10, duration)
    }
}

pub struct FollowPlayerYTrigger;
impl FollowPlayerYTrigger {
    pub fn new(target: Group, speed: f32, delay: f32, offset: f32, max_speed: f32) -> Obj {
        t(1814).set(51, target.0).set(90, speed).set(91, delay).set(92, offset).set(105, max_speed).set(10, 999)
    }
}

// advanced follow settings aren't mapped yet, pass raw (key, value) pairs
pub struct AdvancedFollowTrigger;
impl AdvancedFollowTrigger {
    pub fn new(target: Group, follow: Group, cfg: &[(u16, f32)]) -> Obj {
        cfg.iter().fold(t(3016).set(51, target.0).set(71, follow.0), |o, (k, v)| o.set(*k, v))
    }
}
pub struct EditAdvancedFollowTrigger;
impl EditAdvancedFollowTrigger {
    pub fn new(target: Group, cfg: &[(u16, f32)]) -> Obj {
        cfg.iter().fold(t(3660).set(51, target.0), |o, (k, v)| o.set(*k, v))
    }
}
pub struct RetargetAdvancedFollowTrigger;
impl RetargetAdvancedFollowTrigger {
    pub fn new(target: Group, new_follow: Group) -> Obj { t(3661).set(51, target.0).set(71, new_follow.0) }
}

pub struct ShakeTrigger;
impl ShakeTrigger {
    pub fn new(strength: f32, interval: f32, duration: f32) -> Obj {
        t(1520).set(75, strength).set(84, interval).set(10, duration)
    }
}

// keyframe stuff is the least verified part, see docs/TRIGGER_STATUS.md
pub struct KeyframeTrigger;
impl KeyframeTrigger {
    pub fn new(anim: Group, pos: GridPos, curve: Easing, duration: f32) -> Obj {
        t(3032).at_pos(pos).set(76, anim.0).set(30, e(curve)).set(10, duration)
    }
}
pub struct KeyframeAnimTrigger;
impl KeyframeAnimTrigger {
    pub fn new(anim: Group, target: Group, parent: Group, time_mod: f32, speed_mod: f32) -> Obj {
        t(3033).set(76, anim.0).set(51, target.0).set(71, parent.0).set(520, time_mod).set(521, speed_mod)
    }
}

// ---- area + enter (2.2) ----
// ids are solid, only the shared area keys are confirmed. per-type keys are best guesses

#[derive(Clone)]
pub struct AreaCfg {
    pub length: f32, pub mod_front: f32, pub mod_back: f32,
    pub ease_front: Easing, pub ease_back: Easing, pub relative: bool,
}
impl Default for AreaCfg {
    fn default() -> Self {
        AreaCfg { length: 100.0, mod_front: 1.0, mod_back: 1.0, ease_front: Easing::None, ease_back: Easing::None, relative: false }
    }
}
fn area(id: u16, target: Group, effector: Group, a: &AreaCfg) -> Obj {
    t(id).set(51, target.0).set(71, effector.0).set(222, a.length).set(263, a.mod_front).set(264, a.mod_back)
        .set(243, e(a.ease_front)).set(249, e(a.ease_back)).set_bool(287, a.relative)
}

#[derive(Clone, Default)] pub struct MoveCfg { pub dx: f32, pub dy: f32 }
#[derive(Clone, Default)] pub struct RotateCfg { pub degrees: f32 }
#[derive(Clone)] pub struct ScaleCfg { pub sx: f32, pub sy: f32 }
#[derive(Clone)] pub struct FadeCfg { pub opacity: f32 }
#[derive(Clone)] pub struct TintCfg { pub rgb: (u8, u8, u8), pub opacity: f32 }

fn mv(o: Obj, c: &MoveCfg) -> Obj { o.set(28, u(c.dx)).set(29, u(c.dy)) }
fn rot(o: Obj, c: &RotateCfg) -> Obj { o.set(68, c.degrees) }
fn scl(o: Obj, c: &ScaleCfg) -> Obj { o.set(150, c.sx).set(151, c.sy) }
fn fad(o: Obj, c: &FadeCfg) -> Obj { o.set(35, c.opacity) }
fn tnt(o: Obj, c: &TintCfg) -> Obj { o.rgb(c.rgb.0, c.rgb.1, c.rgb.2).set(35, c.opacity) }

pub struct AreaMoveTrigger;
impl AreaMoveTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: MoveCfg) -> Obj { mv(area(3006, target, eff, &a), &c) } }
pub struct AreaRotateTrigger;
impl AreaRotateTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: RotateCfg) -> Obj { rot(area(3007, target, eff, &a), &c) } }
pub struct AreaScaleTrigger;
impl AreaScaleTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: ScaleCfg) -> Obj { scl(area(3008, target, eff, &a), &c) } }
pub struct AreaFadeTrigger;
impl AreaFadeTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: FadeCfg) -> Obj { fad(area(3009, target, eff, &a), &c) } }
pub struct AreaTintTrigger;
impl AreaTintTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: TintCfg) -> Obj { tnt(area(3010, target, eff, &a), &c) } }

pub struct EditAreaMoveTrigger;
impl EditAreaMoveTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: MoveCfg) -> Obj { mv(area(3011, target, eff, &a), &c) } }
pub struct EditAreaRotateTrigger;
impl EditAreaRotateTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: RotateCfg) -> Obj { rot(area(3012, target, eff, &a), &c) } }
pub struct EditAreaScaleTrigger;
impl EditAreaScaleTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: ScaleCfg) -> Obj { scl(area(3013, target, eff, &a), &c) } }
pub struct EditAreaFadeTrigger;
impl EditAreaFadeTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: FadeCfg) -> Obj { fad(area(3014, target, eff, &a), &c) } }
pub struct EditAreaTintTrigger;
impl EditAreaTintTrigger { pub fn new(target: Group, eff: Group, a: AreaCfg, c: TintCfg) -> Obj { tnt(area(3015, target, eff, &a), &c) } }

// area stop takes the effect id of the area trigger it should kill (key 51 per gdlib)
pub struct AreaStopTrigger;
impl AreaStopTrigger {
    pub fn new(effect_id: u16) -> Obj { t(3024).set(51, effect_id) }
}

fn enter(id: u16, target: Group, mode: TransitionMode, channel: u16) -> Obj {
    t(id).set(51, target.0).set(217, mode as u8).set(344, channel)
}
pub struct MoveEnterTrigger;
impl MoveEnterTrigger { pub fn new(target: Group, mode: TransitionMode, channel: u16) -> Obj { enter(3017, target, mode, channel) } }
pub struct RotateEnterTrigger;
impl RotateEnterTrigger { pub fn new(target: Group, mode: TransitionMode, channel: u16) -> Obj { enter(3018, target, mode, channel) } }
pub struct ScaleEnterTrigger;
impl ScaleEnterTrigger { pub fn new(target: Group, mode: TransitionMode, channel: u16) -> Obj { enter(3019, target, mode, channel) } }
pub struct FadeEnterTrigger;
impl FadeEnterTrigger { pub fn new(target: Group, mode: TransitionMode, channel: u16) -> Obj { enter(3020, target, mode, channel) } }
pub struct TintEnterTrigger;
impl TintEnterTrigger { pub fn new(target: Group, mode: TransitionMode, channel: u16) -> Obj { enter(3021, target, mode, channel) } }
pub struct StopEnterTrigger;
impl StopEnterTrigger { pub fn new(target: Group, mode: TransitionMode, channel: u16) -> Obj { enter(3023, target, mode, channel) } }

// ---- logic, counters, timers, events ----

pub struct SpawnTrigger;
impl SpawnTrigger {
    pub fn new(target: Group, delay: f32, variance: f32) -> Obj {
        let o = t(1268).set(51, target.0).set(63, delay);
        if variance > 0.0 { o.set(556, variance) } else { o }
    }
}

pub struct StopTrigger;
impl StopTrigger {
    pub fn new(target: Group, mode: StopMode) -> Obj {
        let m = match mode { StopMode::Stop => 0, StopMode::Pause => 1, StopMode::Resume => 2 };
        t(1616).set(51, target.0).set(580, m)
    }
}

pub struct ResetTrigger;
impl ResetTrigger {
    pub fn new(target: Group) -> Obj { t(3618).set(51, target.0) }
}

pub struct RandomTrigger;
impl RandomTrigger {
    // chance_a is 0..1, gd stores percent
    pub fn new(a: Group, b: Group, chance_a: f32) -> Obj {
        t(1912).set(51, a.0).set(71, b.0).set(10, chance_a * 100.0)
    }
}

pub struct AdvancedRandomTrigger;
impl AdvancedRandomTrigger {
    pub fn new(groups: &[(Group, u32)]) -> Obj {
        let s = groups.iter().map(|(g, w)| format!("{}.{w}", g.0)).collect::<Vec<_>>().join(".");
        t(2068).set(152, s)
    }
}

pub struct SequenceTrigger;
impl SequenceTrigger {
    // delay between steps is in ms
    pub fn new(steps: &[(Group, u32)], mode: SeqMode) -> Obj {
        let s = steps.iter().map(|(g, d)| format!("{}.{d}", g.0)).collect::<Vec<_>>().join(".");
        t(3607).set(435, s).set(436, mode as u8)
    }
}

pub struct CountTrigger;
impl CountTrigger {
    pub fn new(item: Item, target_count: i32, target: Group, activate: bool, multi: bool) -> Obj {
        t(1611).set(80, item.0).set(77, target_count).set(51, target.0)
            .set_bool(56, activate).set_bool(104, multi)
    }
}

pub struct InstantCountTrigger;
impl InstantCountTrigger {
    pub fn new(item: Item, target_count: i32, target: Group, activate: bool, compare: Compare) -> Obj {
        t(1811).set(80, item.0).set(77, target_count).set(51, target.0)
            .set_bool(56, activate).set(88, compare as u8)
    }
}

pub struct PickupTrigger;
impl PickupTrigger {
    pub fn new(item: Item, delta: i32, override_val: bool) -> Obj {
        t(1817).set(80, item.0).set(77, delta).set_bool(139, override_val)
    }
}

fn op(m: MathOp) -> u8 { m as u8 }
fn rnd(r: RoundMode) -> u8 { r as u8 }

pub struct ItemEditTrigger;
impl ItemEditTrigger {
    // target = (item1 op item2) * mod, item types default to plain items, use .item_types() for timers
    pub fn new(item1: Item, item2: Item, target: Item, m: MathOp, mod1: f32, mod2: f32, round: RoundMode) -> Obj {
        t(3619).set(80, item1.0).set(95, item2.0).set(51, target.0)
            .set(480, op(MathOp::Set)).set(481, op(m)).set(482, op(MathOp::Mul))
            .set(479, mod1).set(483, mod2).set(485, rnd(round))
            .item_types(ItemType::Item, ItemType::Item, ItemType::Item)
    }
}

pub struct ItemCompareTrigger;
impl ItemCompareTrigger {
    pub fn new(item1: Item, item2: Item, true_group: Group, false_group: Group, c: Compare, mod1: f32, mod2: f32, round: RoundMode) -> Obj {
        t(3620).set(80, item1.0).set(95, item2.0).set(51, true_group.0).set(71, false_group.0)
            .set(482, c as u8).set(479, mod1).set(483, mod2).set(485, rnd(round))
            .set(480, op(MathOp::Mul)).set(481, op(MathOp::Mul))
            .item_types(ItemType::Item, ItemType::Item, ItemType::Item)
    }
}

pub struct PersistentItemTrigger;
impl PersistentItemTrigger {
    pub fn new(item: Item, persistent: bool, reset_on_death: bool) -> Obj {
        t(3641).set(80, item.0).set_bool(491, persistent).set_bool(493, reset_on_death)
    }
}

pub struct TimeTrigger;
impl TimeTrigger {
    pub fn new(timer: Item, target_time: f32, target: Group, activate: bool, multi: bool) -> Obj {
        t(3614).set(80, timer.0).set(473, target_time).set_bool(474, true).set(51, target.0)
            .set_bool(56, activate).set_bool(87, multi)
    }
}

pub struct TimeEventTrigger;
impl TimeEventTrigger {
    pub fn new(timer: Item, target: Group, interval: f32, multi: bool) -> Obj {
        t(3615).set(80, timer.0).set(51, target.0).set(473, interval).set_bool(475, multi)
    }
}

pub struct TimeControlTrigger;
impl TimeControlTrigger {
    pub fn new(timer: Item, start: bool, stop: bool) -> Obj {
        t(3617).set(80, timer.0).set_bool(472, stop && !start)
    }
}

pub struct CollisionTrigger;
impl CollisionTrigger {
    pub fn new(a: BlockId, b: BlockId, target: Group, on_exit: bool) -> Obj {
        t(1815).set(80, a.0).set(95, b.0).set(51, target.0).set_bool(56, true).set_bool(93, on_exit)
    }
}

pub struct TouchTrigger;
impl TouchTrigger {
    pub fn new(target: Group, hold: bool, toggle: ToggleMode, dual: DualMode) -> Obj {
        t(1595).set(51, target.0).set_bool(81, hold).set(82, toggle as u8).set_bool(89, matches!(dual, DualMode::Dual))
    }
}

pub struct OnDeathTrigger;
impl OnDeathTrigger {
    pub fn new(target: Group, activate: bool) -> Obj { t(1812).set(51, target.0).set_bool(56, activate) }
}

pub struct EventTrigger;
impl EventTrigger {
    // event ids live in gd_rust::event
    pub fn new(event_id: u16, target: Group) -> Obj { t(3604).set(51, target.0).set(430, event_id) }
}

// ---- camera, shaders, audio, gameplay ----

pub struct CameraZoomTrigger;
impl CameraZoomTrigger {
    pub fn new(zoom: f32, duration: f32, easing: Easing) -> Obj {
        t(1913).set(371, zoom).set(10, duration).set(30, e(easing))
    }
}

pub struct StaticCameraTrigger;
impl StaticCameraTrigger {
    pub fn new(target: Group, exit_static: bool, follow: bool, duration: f32, easing: Easing) -> Obj {
        t(1914).set(71, target.0).set_bool(110, exit_static).set_bool(212, follow)
            .set(10, duration).set(30, e(easing))
    }
}

pub struct CameraOffsetTrigger;
impl CameraOffsetTrigger {
    pub fn new(dx: f32, dy: f32, duration: f32, easing: Easing) -> Obj {
        t(1916).set(28, u(dx)).set(29, u(dy)).set(10, duration).set(30, e(easing))
    }
}

pub struct GameplayOffsetTrigger;
impl GameplayOffsetTrigger {
    pub fn new(x: f32, y: f32, duration: f32, easing: Easing) -> Obj {
        t(2901).set(28, u(x)).set(29, u(y)).set(10, duration).set(30, e(easing))
    }
}

pub struct CameraRotateTrigger;
impl CameraRotateTrigger {
    pub fn new(degrees: f32, duration: f32, easing: Easing, add: bool) -> Obj {
        t(2015).set(68, degrees).set(10, duration).set(30, e(easing)).set_bool(70, add)
    }
}

pub struct CameraEdgeTrigger;
impl CameraEdgeTrigger {
    pub fn new(edge: Edge, target: Group) -> Obj { t(2062).set(164, edge as u8).set(51, target.0) }
}

pub struct CameraModeTrigger;
impl CameraModeTrigger {
    pub fn new(free_mode: bool, easing: Easing) -> Obj {
        t(2925).set_bool(111, free_mode).set(112, 1).set(113, e(easing))
    }
}

pub struct CameraGuideTrigger;
impl CameraGuideTrigger {
    pub fn new(x: f32, y: f32, zoom: f32, preview_opacity: f32) -> Obj {
        t(2016).set(28, u(x)).set(29, u(y)).set(371, zoom).set(506, preview_opacity)
    }
}

#[derive(Clone, Default)]
pub struct ShaderParams {
    pub duration: f32,
    pub speed: f32,
    pub strength: f32,
    // anything shader specific (radius, thickness...) as raw (key, value)
    pub extra: Vec<(u16, f32)>,
}

pub struct ShaderTrigger;
impl ShaderTrigger {
    pub fn new(shader: ShaderType, p: ShaderParams) -> Obj {
        let id = match shader {
            ShaderType::Layers => 2904, ShaderType::ShockWave => 2905, ShaderType::ShockLine => 2907,
            ShaderType::Glitch => 2909, ShaderType::Chromatic => 2910, ShaderType::ChromaticGlitch => 2911,
            ShaderType::Pixelate => 2912, ShaderType::LensCircle => 2913, ShaderType::RadialBlur => 2914,
            ShaderType::Blur => 2915, ShaderType::Bulge => 2916, ShaderType::Pinch => 2917,
            ShaderType::Grayscale => 2919, ShaderType::Sepia => 2920, ShaderType::Invert => 2921,
            ShaderType::HueShift => 2922, ShaderType::EditColor => 2923, ShaderType::SplitScreen => 2924,
        };
        let o = t(id).set(10, p.duration).set(175, p.speed).set(176, p.strength);
        p.extra.iter().fold(o, |o, (k, v)| o.set(*k, v))
    }
}
pub struct ShockWaveTrigger;
impl ShockWaveTrigger { pub fn new(p: ShaderParams) -> Obj { ShaderTrigger::new(ShaderType::ShockWave, p) } }
pub struct ShockLineTrigger;
impl ShockLineTrigger { pub fn new(p: ShaderParams) -> Obj { ShaderTrigger::new(ShaderType::ShockLine, p) } }

pub struct ReverseTrigger;
impl ReverseTrigger { pub fn new() -> Obj { t(1917) } }

// gameplay rotation trigger, only the direction is mapped
pub struct ArrowTrigger;
impl ArrowTrigger {
    pub fn new(direction_degrees: f32) -> Obj { t(2900).set(68, direction_degrees) }
}

pub struct TimeWarpTrigger;
impl TimeWarpTrigger { pub fn new(scale: f32) -> Obj { t(1935).set(120, scale) } }

pub struct BpmTrigger;
impl BpmTrigger {
    pub fn new(bpm: f32, beat_offset: f32) -> Obj { t(3642).set(10, bpm).set(28, beat_offset) }
}

pub struct SfxTrigger;
impl SfxTrigger {
    pub fn new(sfx_id: u32, volume: f32, speed: i32, pitch: i32) -> Obj {
        t(3602).set(392, sfx_id).set(406, volume).set(404, speed).set(405, pitch)
    }
}

pub struct SongTrigger;
impl SongTrigger {
    // offset is ms
    pub fn new(song_id: u32, offset: i32, loop_song: bool) -> Obj {
        t(1934).set(392, song_id).set(408, offset).set_bool(413, loop_song)
    }
}
pub struct EditSongTrigger;
impl EditSongTrigger {
    pub fn new(volume: f32, speed: i32) -> Obj {
        t(3605).set(406, volume).set(404, speed).set(418, 1).set(419, 1)
    }
}

pub struct OptionsTrigger;
impl OptionsTrigger {
    pub fn new(flags: &[(OptionFlag, bool)]) -> Obj {
        flags.iter().fold(t(2899), |o, (f, on)| {
            let key = match f {
                OptionFlag::StreakAdditive => 159, OptionFlag::HideGround => 161, OptionFlag::HideMg => 195,
                OptionFlag::HideP1 => 162, OptionFlag::HideP2 => 163, OptionFlag::DisableControlsP1 => 165,
                OptionFlag::DisableControlsP2 => 199, OptionFlag::UnlinkDualGravity => 160,
                OptionFlag::HideAttempts => 532, OptionFlag::AudioOnDeath => 575, OptionFlag::NoDeathSfx => 576,
            };
            o.set(key, if *on { 1 } else { -1 })
        })
    }
}

// the real trigger only has stop jump/move/rotation/slide, no separate left/right
pub struct PlayerControlTrigger;
impl PlayerControlTrigger {
    pub fn new(stop_jump: bool, stop_move: bool, stop_rot: bool, stop_slide: bool) -> Obj {
        t(1932).set_bool(540, stop_jump).set_bool(541, stop_move).set_bool(542, stop_rot).set_bool(543, stop_slide)
    }
}

pub struct StartPos;
impl StartPos {
    pub fn new(x: f32, y: f32, settings: &crate::Settings) -> Obj {
        Obj::raw(31).at(x, y).set(155, 1).set_tail(&crate::level::settings_string(settings))
    }
}
