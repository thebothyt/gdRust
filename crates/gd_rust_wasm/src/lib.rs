use gd_rust::prelude::*;
use rhai::{Dynamic, Engine, EvalAltResult, Map, Module, INT};
use std::{cell::RefCell, rc::Rc};

type Res<T> = Result<T, Box<EvalAltResult>>;
// rhai reserves `new` and `default`, so scripts keep writing X::new() and we rename them before parsing
fn alias(n: &str) -> String { if n == "new" || n == "default" { format!("{n}_") } else { n.to_string() } }
fn err(msg: &str) -> Box<EvalAltResult> { msg.into() }

// rhai only has i64/f64, so every argument comes in as Dynamic and gets converted here
trait Arg: Sized { fn get(d: &Dynamic) -> Res<Self>; }

fn num(d: &Dynamic) -> Res<f64> {
    d.as_float().or_else(|_| d.as_int().map(|i| i as f64)).map_err(|_| err("expected a number"))
}
fn arr(d: &Dynamic) -> Res<Vec<Dynamic>> {
    d.clone().into_array().map_err(|_| err("expected an array"))
}

macro_rules! num_arg { ($($t:ty),*) => { $( impl Arg for $t { fn get(d: &Dynamic) -> Res<Self> { Ok(num(d)? as $t) } } )* } }
num_arg!(f32, i32, u8, u16, u32);

impl Arg for bool { fn get(d: &Dynamic) -> Res<Self> { d.as_bool().map_err(|_| err("expected true/false")) } }
impl Arg for Group { fn get(d: &Dynamic) -> Res<Self> { Ok(Group(num(d)? as u16)) } }
impl Arg for Color { fn get(d: &Dynamic) -> Res<Self> { Ok(Color(num(d)? as u16)) } }
impl Arg for Item { fn get(d: &Dynamic) -> Res<Self> { Ok(Item(num(d)? as u16)) } }
impl Arg for BlockId { fn get(d: &Dynamic) -> Res<Self> { Ok(BlockId(num(d)? as u16)) } }
impl Arg for GridPos {
    fn get(d: &Dynamic) -> Res<Self> {
        let a = arr(d)?;
        if a.len() != 2 { return Err(err("position is [x, y]")); }
        Ok(GridPos(num(&a[0])? as f32, num(&a[1])? as f32))
    }
}
// [[group, n], [group, n]]
impl Arg for Vec<(Group, u32)> {
    fn get(d: &Dynamic) -> Res<Self> {
        arr(d)?.iter().map(|p| { let p = arr(p)?; Ok((Group::get(&p[0])?, u32::get(&p[1])?)) }).collect()
    }
}
// [[key, value], ...]
impl Arg for Vec<(u16, f32)> {
    fn get(d: &Dynamic) -> Res<Self> {
        arr(d)?.iter().map(|p| { let p = arr(p)?; Ok((u16::get(&p[0])?, f32::get(&p[1])?)) }).collect()
    }
}
impl Arg for Vec<(u16, u16)> {
    fn get(d: &Dynamic) -> Res<Self> {
        arr(d)?.iter().map(|p| { let p = arr(p)?; Ok((u16::get(&p[0])?, u16::get(&p[1])?)) }).collect()
    }
}
impl Arg for Vec<Group> {
    fn get(d: &Dynamic) -> Res<Self> { arr(d)?.iter().map(Group::get).collect() }
}
impl Arg for Vec<(OptionFlag, bool)> {
    fn get(d: &Dynamic) -> Res<Self> {
        arr(d)?.iter().map(|p| { let p = arr(p)?; Ok((OptionFlag::get(&p[0])?, bool::get(&p[1])?)) }).collect()
    }
}
impl Arg for Vec<Flag> {
    fn get(d: &Dynamic) -> Res<Self> { arr(d)?.iter().map(Flag::get).collect() }
}

// every enum is exposed to scripts as Name::Variant (an int under the hood)
macro_rules! enums {
    ($($t:ident : [$($v:ident),* $(,)?]),* $(,)?) => {
        $( impl Arg for $t {
            fn get(d: &Dynamic) -> Res<Self> {
                const ALL: &[$t] = &[$($t::$v),*];
                let i = d.as_int().map_err(|_| err(concat!("expected ", stringify!($t), "::...")))?;
                ALL.get(i as usize).copied().ok_or_else(|| err(concat!("bad ", stringify!($t), " value")))
            }
        } )*
        fn register_enums(engine: &mut Engine) {
            $( {
                let mut m = Module::new();
                let mut i: INT = 0;
                $( m.set_var(stringify!($v), i); i += 1; )*
                let _ = i;
                engine.register_static_module(stringify!($t), m.into());
            } )*
        }
    };
}
enums! {
    Easing: [None, EaseInOut, EaseIn, EaseOut, ElasticInOut, ElasticIn, ElasticOut, BounceInOut, BounceIn, BounceOut,
             ExpoInOut, ExpoIn, ExpoOut, SineInOut, SineIn, SineOut, BackInOut, BackIn, BackOut],
    ZLayer: [B5, B4, B3, B2, B1, T1, T2, T3, T4],
    Flag: [NoTouch, DontFade, DontEnter, HighDetail, NoEffects, NoGlow],
    Mode: [Cube, Ship, Ball, Ufo, Wave, Robot, Spider, Swing],
    Speed: [Half, Normal, Double, Triple, Quad],
    PulseMode: [Color, Group],
    StopMode: [Stop, Pause, Resume],
    SeqMode: [Default, Loop, Snap],
    Compare: [Equal, Larger, Smaller],
    MathOp: [Set, Add, Sub, Mul, Div],
    RoundMode: [None, Round, Floor, Ceil],
    ToggleMode: [Normal, On, Off],
    DualMode: [Normal, Dual],
    Edge: [Left, Right, Up, Down],
    ItemType: [Item, Timer, Points, Time],
    TransitionMode: [Both, Enter, Exit],
    ShaderType: [Layers, ShockWave, ShockLine, Glitch, Chromatic, ChromaticGlitch, Pixelate, LensCircle,
                 RadialBlur, Blur, Bulge, Pinch, Grayscale, Sepia, Invert, HueShift, EditColor, SplitScreen],
    OptionFlag: [StreakAdditive, HideGround, HideMg, HideP1, HideP2, DisableControlsP1, DisableControlsP2,
                 UnlinkDualGravity, HideAttempts, AudioOnDeath, NoDeathSfx],
}

// config maps: #{ length: 300, relative: true }, anything missing falls back to the default
fn mf(m: &Map, k: &str, def: f32) -> Res<f32> { m.get(k).map_or(Ok(def), |v| Ok(num(v)? as f32)) }
fn mb(m: &Map, k: &str, def: bool) -> Res<bool> { m.get(k).map_or(Ok(def), |v| bool::get(v)) }
fn me(m: &Map, k: &str) -> Res<Easing> { m.get(k).map_or(Ok(Easing::None), Easing::get) }
fn mg(m: &Map, k: &str) -> Res<Group> { m.get(k).map_or(Ok(Group(0)), Group::get) }
fn mc(m: &Map, k: &str) -> Res<Color> { m.get(k).map_or(Ok(Color(0)), Color::get) }

fn area_cfg(m: &Map) -> Res<AreaCfg> {
    let d = AreaCfg::default();
    Ok(AreaCfg {
        length: mf(m, "length", d.length)?, mod_front: mf(m, "mod_front", d.mod_front)?, mod_back: mf(m, "mod_back", d.mod_back)?,
        ease_front: me(m, "ease_front")?, ease_back: me(m, "ease_back")?, relative: mb(m, "relative", false)?,
    })
}

macro_rules! ctor {
    ($m:ident, $name:literal, ($($a:ident : $t:ty),*) => $body:expr) => {
        $m.set_native_fn(alias($name), |$($a: Dynamic),*| -> Res<Obj> {
            $( let $a = <$t as Arg>::get(&$a)?; )*
            Ok($body)
        });
    };
}
macro_rules! group {
    ($engine:ident, $name:expr, |$m:ident| $body:block) => {{
        let mut $m = Module::new();
        $body
        $engine.register_static_module($name, $m.into());
    }};
}
// shorthand for the common (x, y) constructors
macro_rules! xy {
    ($m:ident, $($name:literal => $f:path),*) => {
        $( ctor!($m, $name, (x: f32, y: f32) => $f(x, y)); )*
    };
}

fn register_objects(e: &mut Engine) {
    group!(e, "Block", |m| {
        xy!(m, "default" => Block::default, "slab" => Block::slab);
        ctor!(m, "slope", (x: f32, y: f32, a: f32) => Block::slope(x, y, a));
        ctor!(m, "custom", (id: u16, x: f32, y: f32) => Block::custom(id, x, y));
    });
    group!(e, "Spike", |m| {
        xy!(m, "regular" => Spike::regular, "small" => Spike::small, "ground" => Spike::ground);
        ctor!(m, "custom", (id: u16, x: f32, y: f32) => Spike::custom(id, x, y));
    });
    group!(e, "Sawblade", |m| { ctor!(m, "new", (x: f32, y: f32, r: f32) => Sawblade::new(x, y, r)); });
    group!(e, "SmartBlock", |m| {
        xy!(m, "d_block" => SmartBlock::d_block, "j_block" => SmartBlock::j_block,
               "s_block" => SmartBlock::s_block, "h_block" => SmartBlock::h_block);
    });
    group!(e, "Orb", |m| {
        xy!(m, "yellow" => Orb::yellow, "pink" => Orb::pink, "red" => Orb::red, "blue" => Orb::blue,
               "green" => Orb::green, "black" => Orb::black, "spider" => Orb::spider);
        ctor!(m, "dash", (x: f32, y: f32, a: f32) => Orb::dash(x, y, a));
    });
    group!(e, "Pad", |m| {
        xy!(m, "yellow" => Pad::yellow, "pink" => Pad::pink, "red" => Pad::red,
               "blue" => Pad::blue, "spider" => Pad::spider);
    });
    group!(e, "UserCoin", |m| { xy!(m, "new" => UserCoin::new); });
    group!(e, "ItemDisplay", |m| { ctor!(m, "new", (x: f32, y: f32, i: Item, timer: bool) => ItemDisplay::new(x, y, i, timer)); });
    group!(e, "CollisionBlock", |m| {
        ctor!(m, "new", (x: f32, y: f32, id: BlockId, dynamic: bool) => CollisionBlock::new(x, y, id, dynamic));
    });
    group!(e, "Portal", |m| {
        ctor!(m, "gamemode", (x: f32, y: f32, g: Mode) => Portal::gamemode(x, y, g));
        ctor!(m, "speed", (x: f32, y: f32, s: Speed) => Portal::speed(x, y, s));
        ctor!(m, "gravity", (x: f32, y: f32, b: bool) => Portal::gravity(x, y, b));
        ctor!(m, "size", (x: f32, y: f32, b: bool) => Portal::size(x, y, b));
        ctor!(m, "dual", (x: f32, y: f32, b: bool) => Portal::dual(x, y, b));
        ctor!(m, "mirror", (x: f32, y: f32, b: bool) => Portal::mirror(x, y, b));
        ctor!(m, "teleport", (a: GridPos, b: GridPos) => Portal::teleport(a, b));
    });
    group!(e, "Deco", |m| {
        ctor!(m, "custom", (id: u16, x: f32, y: f32) => Deco::custom(id, x, y));
        ctor!(m, "pixel", (x: f32, y: f32, id: u16) => Deco::pixel(x, y, id));
        ctor!(m, "foliage", (x: f32, y: f32, id: u16) => Deco::foliage(x, y, id));
        ctor!(m, "glow", (x: f32, y: f32, sx: f32, sy: f32) => Deco::glow(x, y, sx, sy));
        xy!(m, "chain" => Deco::chain, "detail_border" => Deco::detail_border, "gradient" => Deco::gradient);
        // particle_emitter(x, y, [[index, value], ...])
        ctor!(m, "particle_emitter", (x: f32, y: f32, kv: Vec<(u16, f32)>) =>
            Deco::particle_emitter(x, y, kv.iter().fold(ParticleConfig::default(), |c, (i, v)| c.with(*i as usize, *v))));
    });
}

fn register_triggers(e: &mut Engine) {
    group!(e, "ColorTrigger", |m| {
        ctor!(m, "new", (c: Color, r: u8, g: u8, b: u8, a: f32, d: f32, blend: bool) => ColorTrigger::new(c, (r, g, b, a), d, blend));
    });
    group!(e, "PulseTrigger", |m| {
        ctor!(m, "new", (t: u16, mode: PulseMode, i: f32, h: f32, o: f32) => PulseTrigger::new(t, mode, i, h, o));
    });
    group!(e, "AlphaTrigger", |m| { ctor!(m, "new", (g: Group, o: f32, d: f32) => AlphaTrigger::new(g, o, d)); });
    group!(e, "ToggleTrigger", |m| { ctor!(m, "new", (g: Group, on: bool) => ToggleTrigger::new(g, on)); });
    group!(e, "AnimateTrigger", |m| { ctor!(m, "new", (g: Group, a: i32) => AnimateTrigger::new(g, a)); });
    group!(e, "ChangeBgTrigger", |m| { ctor!(m, "new", (id: u16, d: f32) => ChangeBgTrigger::new(id, d)); });
    group!(e, "ChangeGroundTrigger", |m| { ctor!(m, "new", (id: u16, d: f32) => ChangeGroundTrigger::new(id, d)); });
    group!(e, "ChangeMgTrigger", |m| { ctor!(m, "new", (id: u16, d: f32) => ChangeMgTrigger::new(id, d)); });
    group!(e, "GradientTrigger", |m| {
        m.set_native_fn("new_", |id: Dynamic, c: Map| -> Res<Obj> {
            Ok(GradientTrigger::new(u16::get(&id)?, GradientCfg {
                bl: mg(&c, "bl")?, br: mg(&c, "br")?, tl: mg(&c, "tl")?, tr: mg(&c, "tr")?,
                color1: mc(&c, "color1")?, color2: mc(&c, "color2")?,
                vertex_mode: mb(&c, "vertex_mode", true)?, blending: mb(&c, "blending", false)?,
                layer: mf(&c, "layer", 0.0)? as u8,
            }))
        });
    });
    group!(e, "SpawnParticleTrigger", |m| {
        ctor!(m, "new", (g: Group, p: Group, o: GridPos, s: f32, a: f32) => SpawnParticleTrigger::new(g, p, o, s, a));
    });
    group!(e, "LinkVisibleTrigger", |m| { ctor!(m, "new", (g: Group) => LinkVisibleTrigger::new(g)); });

    group!(e, "MoveTrigger", |m| {
        ctor!(m, "new", (g: Group, dx: f32, dy: f32, d: f32, ea: Easing) => MoveTrigger::new(g, dx, dy, d, ea));
    });
    group!(e, "RotateTrigger", |m| {
        ctor!(m, "new", (g: Group, c: Group, deg: f32, t: i32, d: f32, ea: Easing) => RotateTrigger::new(g, c, deg, t, d, ea));
    });
    group!(e, "ScaleTrigger", |m| {
        ctor!(m, "new", (g: Group, c: Group, sx: f32, sy: f32, d: f32, ea: Easing) => ScaleTrigger::new(g, c, sx, sy, d, ea));
    });
    group!(e, "FollowTrigger", |m| {
        ctor!(m, "new", (g: Group, f: Group, x: f32, y: f32, d: f32) => FollowTrigger::new(g, f, x, y, d));
    });
    group!(e, "FollowPlayerYTrigger", |m| {
        ctor!(m, "new", (g: Group, s: f32, d: f32, o: f32, mx: f32) => FollowPlayerYTrigger::new(g, s, d, o, mx));
    });
    group!(e, "AdvancedFollowTrigger", |m| {
        ctor!(m, "new", (g: Group, f: Group, cfg: Vec<(u16, f32)>) => AdvancedFollowTrigger::new(g, f, &cfg));
    });
    group!(e, "EditAdvancedFollowTrigger", |m| {
        ctor!(m, "new", (g: Group, cfg: Vec<(u16, f32)>) => EditAdvancedFollowTrigger::new(g, &cfg));
    });
    group!(e, "RetargetAdvancedFollowTrigger", |m| {
        ctor!(m, "new", (g: Group, f: Group) => RetargetAdvancedFollowTrigger::new(g, f));
    });
    group!(e, "ShakeTrigger", |m| { ctor!(m, "new", (s: f32, i: f32, d: f32) => ShakeTrigger::new(s, i, d)); });
    group!(e, "KeyframeTrigger", |m| {
        ctor!(m, "new", (g: Group, p: GridPos, c: Easing, d: f32) => KeyframeTrigger::new(g, p, c, d));
    });
    group!(e, "KeyframeAnimTrigger", |m| {
        ctor!(m, "new", (a: Group, t: Group, p: Group, tm: f32, sm: f32) => KeyframeAnimTrigger::new(a, t, p, tm, sm));
    });

    // area triggers take two maps: the shared area settings and the per-type settings
    macro_rules! area_trig {
        ($($name:literal => $ty:ident, $cfg:ident, |$c:ident| $build:expr);* $(;)?) => { $(
            group!(e, $name, |m| {
                m.set_native_fn("new_", |t: Dynamic, ef: Dynamic, a: Map, $c: Map| -> Res<Obj> {
                    Ok($ty::new(Group::get(&t)?, Group::get(&ef)?, area_cfg(&a)?, $build))
                });
            });
        )* };
    }
    area_trig! {
        "AreaMoveTrigger" => AreaMoveTrigger, MoveCfg, |c| MoveCfg { dx: mf(&c, "dx", 0.0)?, dy: mf(&c, "dy", 0.0)? };
        "AreaRotateTrigger" => AreaRotateTrigger, RotateCfg, |c| RotateCfg { degrees: mf(&c, "degrees", 0.0)? };
        "AreaScaleTrigger" => AreaScaleTrigger, ScaleCfg, |c| ScaleCfg { sx: mf(&c, "sx", 1.0)?, sy: mf(&c, "sy", 1.0)? };
        "AreaFadeTrigger" => AreaFadeTrigger, FadeCfg, |c| FadeCfg { opacity: mf(&c, "opacity", 1.0)? };
        "AreaTintTrigger" => AreaTintTrigger, TintCfg, |c| TintCfg { rgb: (mf(&c, "r", 255.0)? as u8, mf(&c, "g", 255.0)? as u8, mf(&c, "b", 255.0)? as u8), opacity: mf(&c, "opacity", 1.0)? };
        "EditAreaMoveTrigger" => EditAreaMoveTrigger, MoveCfg, |c| MoveCfg { dx: mf(&c, "dx", 0.0)?, dy: mf(&c, "dy", 0.0)? };
        "EditAreaRotateTrigger" => EditAreaRotateTrigger, RotateCfg, |c| RotateCfg { degrees: mf(&c, "degrees", 0.0)? };
        "EditAreaScaleTrigger" => EditAreaScaleTrigger, ScaleCfg, |c| ScaleCfg { sx: mf(&c, "sx", 1.0)?, sy: mf(&c, "sy", 1.0)? };
        "EditAreaFadeTrigger" => EditAreaFadeTrigger, FadeCfg, |c| FadeCfg { opacity: mf(&c, "opacity", 1.0)? };
        "EditAreaTintTrigger" => EditAreaTintTrigger, TintCfg, |c| TintCfg { rgb: (mf(&c, "r", 255.0)? as u8, mf(&c, "g", 255.0)? as u8, mf(&c, "b", 255.0)? as u8), opacity: mf(&c, "opacity", 1.0)? };
    }
    group!(e, "AreaStopTrigger", |m| { ctor!(m, "new", (id: u16) => AreaStopTrigger::new(id)); });
    macro_rules! enter_trig {
        ($($name:literal => $ty:ident),*) => { $(
            group!(e, $name, |m| { ctor!(m, "new", (g: Group, mode: TransitionMode, ch: u16) => $ty::new(g, mode, ch)); });
        )* };
    }
    enter_trig!("MoveEnterTrigger" => MoveEnterTrigger, "RotateEnterTrigger" => RotateEnterTrigger,
                "ScaleEnterTrigger" => ScaleEnterTrigger, "FadeEnterTrigger" => FadeEnterTrigger,
                "TintEnterTrigger" => TintEnterTrigger, "StopEnterTrigger" => StopEnterTrigger);

    group!(e, "SpawnTrigger", |m| { ctor!(m, "new", (g: Group, d: f32, v: f32) => SpawnTrigger::new(g, d, v)); });
    group!(e, "StopTrigger", |m| { ctor!(m, "new", (g: Group, mode: StopMode) => StopTrigger::new(g, mode)); });
    group!(e, "ResetTrigger", |m| { ctor!(m, "new", (g: Group) => ResetTrigger::new(g)); });
    group!(e, "RandomTrigger", |m| { ctor!(m, "new", (a: Group, b: Group, c: f32) => RandomTrigger::new(a, b, c)); });
    group!(e, "AdvancedRandomTrigger", |m| { ctor!(m, "new", (g: Vec<(Group, u32)>) => AdvancedRandomTrigger::new(&g)); });
    group!(e, "SequenceTrigger", |m| { ctor!(m, "new", (g: Vec<(Group, u32)>, mode: SeqMode) => SequenceTrigger::new(&g, mode)); });
    group!(e, "CountTrigger", |m| {
        ctor!(m, "new", (i: Item, n: i32, g: Group, a: bool, mu: bool) => CountTrigger::new(i, n, g, a, mu));
    });
    group!(e, "InstantCountTrigger", |m| {
        ctor!(m, "new", (i: Item, n: i32, g: Group, a: bool, c: Compare) => InstantCountTrigger::new(i, n, g, a, c));
    });
    group!(e, "PickupTrigger", |m| { ctor!(m, "new", (i: Item, d: i32, o: bool) => PickupTrigger::new(i, d, o)); });
    group!(e, "ItemEditTrigger", |m| {
        ctor!(m, "new", (a: Item, b: Item, t: Item, op: MathOp, m1: f32, m2: f32, r: RoundMode) => ItemEditTrigger::new(a, b, t, op, m1, m2, r));
    });
    group!(e, "ItemCompareTrigger", |m| {
        ctor!(m, "new", (a: Item, b: Item, t: Group, f: Group, c: Compare, m1: f32, m2: f32, r: RoundMode) => ItemCompareTrigger::new(a, b, t, f, c, m1, m2, r));
    });
    group!(e, "PersistentItemTrigger", |m| { ctor!(m, "new", (i: Item, p: bool, r: bool) => PersistentItemTrigger::new(i, p, r)); });
    group!(e, "TimeTrigger", |m| {
        ctor!(m, "new", (i: Item, t: f32, g: Group, a: bool, mu: bool) => TimeTrigger::new(i, t, g, a, mu));
    });
    group!(e, "TimeEventTrigger", |m| { ctor!(m, "new", (i: Item, g: Group, t: f32, mu: bool) => TimeEventTrigger::new(i, g, t, mu)); });
    group!(e, "TimeControlTrigger", |m| { ctor!(m, "new", (i: Item, s: bool, p: bool) => TimeControlTrigger::new(i, s, p)); });
    group!(e, "CollisionTrigger", |m| {
        ctor!(m, "new", (a: BlockId, b: BlockId, g: Group, x: bool) => CollisionTrigger::new(a, b, g, x));
    });
    group!(e, "TouchTrigger", |m| {
        ctor!(m, "new", (g: Group, h: bool, t: ToggleMode, d: DualMode) => TouchTrigger::new(g, h, t, d));
    });
    group!(e, "OnDeathTrigger", |m| { ctor!(m, "new", (g: Group, a: bool) => OnDeathTrigger::new(g, a)); });
    group!(e, "EventTrigger", |m| { ctor!(m, "new", (ev: u16, g: Group) => EventTrigger::new(ev, g)); });

    group!(e, "CameraZoomTrigger", |m| { ctor!(m, "new", (z: f32, d: f32, ea: Easing) => CameraZoomTrigger::new(z, d, ea)); });
    group!(e, "StaticCameraTrigger", |m| {
        ctor!(m, "new", (g: Group, x: bool, f: bool, d: f32, ea: Easing) => StaticCameraTrigger::new(g, x, f, d, ea));
    });
    group!(e, "CameraOffsetTrigger", |m| { ctor!(m, "new", (x: f32, y: f32, d: f32, ea: Easing) => CameraOffsetTrigger::new(x, y, d, ea)); });
    group!(e, "GameplayOffsetTrigger", |m| { ctor!(m, "new", (x: f32, y: f32, d: f32, ea: Easing) => GameplayOffsetTrigger::new(x, y, d, ea)); });
    group!(e, "CameraRotateTrigger", |m| { ctor!(m, "new", (r: f32, d: f32, ea: Easing, a: bool) => CameraRotateTrigger::new(r, d, ea, a)); });
    group!(e, "CameraEdgeTrigger", |m| { ctor!(m, "new", (ed: Edge, g: Group) => CameraEdgeTrigger::new(ed, g)); });
    group!(e, "CameraModeTrigger", |m| { ctor!(m, "new", (f: bool, ea: Easing) => CameraModeTrigger::new(f, ea)); });
    group!(e, "CameraGuideTrigger", |m| { ctor!(m, "new", (x: f32, y: f32, z: f32, o: f32) => CameraGuideTrigger::new(x, y, z, o)); });

    fn shader_params(c: &Map) -> Res<ShaderParams> {
        Ok(ShaderParams { duration: mf(c, "duration", 0.0)?, speed: mf(c, "speed", 0.0)?, strength: mf(c, "strength", 0.0)?, extra: vec![] })
    }
    group!(e, "ShaderTrigger", |m| {
        m.set_native_fn("new_", |s: Dynamic, c: Map| -> Res<Obj> { Ok(ShaderTrigger::new(ShaderType::get(&s)?, shader_params(&c)?)) });
    });
    group!(e, "ShockWaveTrigger", |m| { m.set_native_fn("new_", |c: Map| -> Res<Obj> { Ok(ShockWaveTrigger::new(shader_params(&c)?)) }); });
    group!(e, "ShockLineTrigger", |m| { m.set_native_fn("new_", |c: Map| -> Res<Obj> { Ok(ShockLineTrigger::new(shader_params(&c)?)) }); });
    group!(e, "ReverseTrigger", |m| { ctor!(m, "new", () => ReverseTrigger::new()); });
    group!(e, "ArrowTrigger", |m| { ctor!(m, "new", (d: f32) => ArrowTrigger::new(d)); });
    group!(e, "TimeWarpTrigger", |m| { ctor!(m, "new", (s: f32) => TimeWarpTrigger::new(s)); });
    group!(e, "BpmTrigger", |m| { ctor!(m, "new", (b: f32, o: f32) => BpmTrigger::new(b, o)); });
    group!(e, "SfxTrigger", |m| { ctor!(m, "new", (id: u32, v: f32, s: i32, p: i32) => SfxTrigger::new(id, v, s, p)); });
    group!(e, "SongTrigger", |m| { ctor!(m, "new", (id: u32, o: i32, l: bool) => SongTrigger::new(id, o, l)); });
    group!(e, "EditSongTrigger", |m| { ctor!(m, "new", (v: f32, s: i32) => EditSongTrigger::new(v, s)); });
    group!(e, "OptionsTrigger", |m| { ctor!(m, "new", (f: Vec<(OptionFlag, bool)>) => OptionsTrigger::new(&f)); });
    group!(e, "PlayerControlTrigger", |m| {
        ctor!(m, "new", (j: bool, mv: bool, r: bool, s: bool) => PlayerControlTrigger::new(j, mv, r, s));
    });

    // event ids and particle slots as plain int constants: Event::YELLOW_ORB, Particle::SPEED
    let mut ev = Module::new();
    macro_rules! consts { ($m:ident, $ns:ident, [$($n:ident),*]) => { $( $m.set_var(stringify!($n), $ns::$n as INT); )* } }
    consts!(ev, event, [NONE, TINY_LANDING, FEATHER_LANDING, SOFT_LANDING, NORMAL_LANDING, HARD_LANDING, HIT_HEAD,
        ORB_TOUCHED, ORB_ACTIVATED, PAD_ACTIVATED, GRAVITY_INVERTED, GRAVITY_RESTORED, NORMAL_JUMP, ROBOT_BOOST_START,
        ROBOT_BOOST_STOP, UFO_JUMP, SHIP_BOOST_START, SHIP_BOOST_END, SPIDER_TELEPORT, BALL_SWITCH, SWING_SWITCH,
        WAVE_PUSH, WAVE_RELEASE, DASH_START, DASH_STOP, TELEPORTED, YELLOW_ORB, PINK_ORB, RED_ORB, GREEN_ORB,
        YELLOW_PAD, PINK_PAD, RED_PAD, CHECKPOINT, USER_COIN, PICKUP_ITEM, JUMP_PUSH, JUMP_RELEASE, LEFT_PUSH,
        LEFT_RELEASE, RIGHT_PUSH, RIGHT_RELEASE]);
    e.register_static_module("Event", ev.into());
    let mut pm = Module::new();
    consts!(pm, particle, [MAX_PARTICLES, DURATION, LIFETIME, LIFETIME_VAR, EMISSION, ANGLE, ANGLE_VAR, SPEED, SPEED_VAR,
        POSVAR_X, POSVAR_Y, GRAVITY_X, GRAVITY_Y, START_SIZE, START_SIZE_VAR, START_SPIN, START_R, START_G, START_B,
        START_A, END_SIZE, END_SPIN, END_R, END_G, END_B, END_A, FADE_IN, FADE_OUT, ADDITIVE]);
    e.register_static_module("Particle", pm.into());
}

// builder methods + level functions
fn register_methods(e: &mut Engine, level: Rc<RefCell<Level>>) {
    e.register_type_with_name::<Obj>("Obj");

    macro_rules! method {
        ($name:literal, ($($a:ident : $t:ty),*) => |$o:ident| $body:expr) => {
            e.register_fn($name, |$o: Obj, $($a: Dynamic),*| -> Res<Obj> {
                $( let $a = <$t as Arg>::get(&$a)?; )*
                Ok($body)
            });
        };
    }
    method!("at", (x: f32, y: f32) => |o| o.at(x, y));
    method!("spawn_triggered", (m: bool) => |o| o.spawn_triggered(m));
    method!("multi_triggered", (m: bool) => |o| o.multi_triggered(m));
    method!("touch_triggered", (m: bool) => |o| o.touch_triggered(m));
    method!("add_groups", (g: Vec<Group>) => |o| o.add_groups(&g));
    method!("set_parent", (g: Group) => |o| o.set_parent(g));
    method!("with_color", (c: Color) => |o| o.with_color(c));
    method!("with_detail_color", (c: Color) => |o| o.with_detail_color(c));
    method!("with_hsv", (h: i32, s: f32, b: f32, sc: bool, bc: bool) => |o| o.with_hsv(h, s, b, sc, bc));
    method!("with_z", (l: ZLayer, n: i32) => |o| o.with_z(l, n));
    method!("with_transform", (sx: f32, sy: f32, r: f32) => |o| o.with_transform(sx, sy, r));
    method!("with_warp", (x: f32, y: f32) => |o| o.with_warp(x, y));
    method!("set_editor_layer", (a: i32, b: i32) => |o| o.set_editor_layer(a, b));
    method!("set_flags", (f: Vec<Flag>) => |o| o.set_flags(&f));
    method!("remap", (p: Vec<(u16, u16)>) => |o| o.remap(&p));
    method!("easing_rate", (r: f32) => |o| o.easing_rate(r));
    method!("silent", (v: bool) => |o| o.silent(v));
    method!("rgb", (r: u8, g: u8, b: u8) => |o| o.rgb(r, g, b));
    method!("p1", (v: bool) => |o| o.p1(v));
    method!("p2", (v: bool) => |o| o.p2(v));
    method!("channel", (c: u8) => |o| o.channel(c));
    method!("lock_player", (x: bool, y: bool) => |o| o.lock_player(x, y));
    method!("lock_camera", (x: bool, y: bool) => |o| o.lock_camera(x, y));
    method!("target_mode", (t: Group, c: Group, d: bool) => |o| o.target_mode(t, c, d));
    method!("small_step", (v: bool) => |o| o.small_step(v));
    method!("pulse_hsv", (h: i32, s: f32, b: f32, sc: bool, bc: bool) => |o| o.pulse_hsv(h, s, b, sc, bc));
    method!("item_types", (a: ItemType, b: ItemType, t: ItemType) => |o| o.item_types(a, b, t));
    method!("set", (k: u16, v: f32) => |o| o.set(k, v));

    let l = level.clone(); e.register_fn("next_group", move || l.borrow_mut().next_group().0 as INT);
    let l = level.clone(); e.register_fn("next_color", move || l.borrow_mut().next_color().0 as INT);
    let l = level.clone(); e.register_fn("next_item", move || l.borrow_mut().next_item().0 as INT);
    let l = level.clone(); e.register_fn("next_block_id", move || l.borrow_mut().next_block_id().0 as INT);
    let l = level.clone(); e.register_fn("next_remap_id", move || l.borrow_mut().next_remap_id() as INT);
    let l = level.clone(); e.register_fn("add", move |o: Obj| { l.borrow_mut().add(o); });
    let l = level.clone(); e.register_fn("add_all", move |os: rhai::Array| -> Res<()> {
        for o in os { l.borrow_mut().add(o.try_cast::<Obj>().ok_or_else(|| err("add_all wants an array of objects"))?); }
        Ok(())
    });
    let l = level.clone(); e.register_fn("set_name", move |s: &str| l.borrow_mut().name = s.to_string());
    let l = level.clone(); e.register_fn("set_author", move |s: &str| l.borrow_mut().author = s.to_string());
    let l = level.clone(); e.register_fn("set_mode", move |m: Dynamic| -> Res<()> { l.borrow_mut().settings.gamemode = Mode::get(&m)?; Ok(()) });
    let l = level.clone(); e.register_fn("set_speed", move |s: Dynamic| -> Res<()> { l.borrow_mut().settings.speed = Speed::get(&s)?; Ok(()) });
    let l = level.clone(); e.register_fn("set_mini", move |b: bool| l.borrow_mut().settings.mini = b);
    let l = level.clone(); e.register_fn("set_dual", move |b: bool| l.borrow_mut().settings.dual = b);
    let l = level.clone(); e.register_fn("set_platformer", move |b: bool| l.borrow_mut().settings.platformer = b);
    let l = level.clone(); e.register_fn("set_song", move |id: INT| l.borrow_mut().settings.song_id = id as u32);

    let l = level.clone();
    e.register_fn("start_pos", move |x: Dynamic, y: Dynamic| -> Res<Obj> {
        Ok(StartPos::new(f32::get(&x)?, f32::get(&y)?, &l.borrow().settings))
    });
}

pub struct Built { pub gmd: String, pub objects: usize, pub log: String }

pub fn compile(src: &str) -> Result<Built, String> {
    let level = Rc::new(RefCell::new(Level::new("gdrust level")));
    let log = Rc::new(RefCell::new(String::new()));
    let mut engine = Engine::new();
    engine.set_max_operations(20_000_000);
    engine.set_max_call_levels(64);
    let lg = log.clone();
    engine.on_print(move |s| { let mut l = lg.borrow_mut(); l.push_str(s); l.push('\n'); });
    register_enums(&mut engine);
    register_objects(&mut engine);
    register_triggers(&mut engine);
    register_methods(&mut engine, level.clone());

    let src = src.replace("::new(", "::new_(").replace("::default(", "::default_(");
    engine.run(&src).map_err(|e| e.to_string())?;
    let lv = level.borrow();
    let out = Built { gmd: lv.to_gmd_string(), objects: lv.object_count(), log: log.borrow().clone() };
    Ok(out)
}

// ---- plain C ABI so the page doesn't need wasm-bindgen ----
thread_local! {
    static OUT: RefCell<(bool, usize, String, Vec<u8>)> = RefCell::new((false, 0, String::new(), vec![]));
}

#[no_mangle]
pub extern "C" fn gd_alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len);
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

#[no_mangle]
pub extern "C" fn gd_free(ptr: *mut u8, len: usize) {
    unsafe { drop(Vec::from_raw_parts(ptr, 0, len)); }
}

// returns the byte length of the result, then read it with gd_out_ptr
#[no_mangle]
pub extern "C" fn gd_compile(ptr: *const u8, len: usize) -> usize {
    let src = unsafe { std::slice::from_raw_parts(ptr, len) };
    let src = String::from_utf8_lossy(src).into_owned();
    let (ok, n, log, bytes) = match compile(&src) {
        Ok(b) => (true, b.objects, b.log, b.gmd.into_bytes()),
        Err(e) => (false, 0, String::new(), e.into_bytes()),
    };
    OUT.with(|o| *o.borrow_mut() = (ok, n, log, bytes));
    OUT.with(|o| o.borrow().3.len())
}
#[no_mangle] pub extern "C" fn gd_out_ptr() -> *const u8 { OUT.with(|o| o.borrow().3.as_ptr()) }
#[no_mangle] pub extern "C" fn gd_out_ok() -> u32 { OUT.with(|o| o.borrow().0 as u32) }
#[no_mangle] pub extern "C" fn gd_out_objects() -> u32 { OUT.with(|o| o.borrow().1 as u32) }
#[no_mangle] pub extern "C" fn gd_log_len() -> usize { OUT.with(|o| o.borrow().2.len()) }
#[no_mangle] pub extern "C" fn gd_log_ptr() -> *const u8 { OUT.with(|o| o.borrow().2.as_ptr()) }
