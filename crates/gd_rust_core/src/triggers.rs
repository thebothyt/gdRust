use crate::obj::*;

fn ease(e: Easing) -> i32 { e as i32 }

pub struct MoveTrigger;
impl MoveTrigger {
    // dx/dy in blocks. gd moves in units (30 = 1 block)
    pub fn new(target: Group, dx: f32, dy: f32, duration: f32, easing: Easing) -> Obj {
        Obj::raw(901).set(51, target.0).set(28, dx * 30.0).set(29, dy * 30.0)
            .set(10, duration).set(30, ease(easing))
    }
}

pub struct SpawnTrigger;
impl SpawnTrigger {
    pub fn new(target: Group, delay: f32, _variance: f32) -> Obj {
        Obj::raw(1268).set(51, target.0).set(63, delay)
    }
}

pub trait Remap {
    fn remap(self, pairs: &[(u16, u16)]) -> Obj;
}
impl Remap for Obj {
    fn remap(self, pairs: &[(u16, u16)]) -> Obj {
        let s = pairs.iter().map(|(a, b)| format!("{a}.{b}")).collect::<Vec<_>>().join(".");
        self.set(442, s)
    }
}

pub struct ToggleTrigger;
impl ToggleTrigger {
    pub fn new(target: Group, activate: bool) -> Obj {
        Obj::raw(1049).set(51, target.0).set_bool(56, activate)
    }
}

pub struct AlphaTrigger;
impl AlphaTrigger {
    pub fn new(target: Group, opacity: f32, duration: f32) -> Obj {
        Obj::raw(1007).set(51, target.0).set(35, opacity).set(10, duration)
    }
}

pub struct ColorTrigger;
impl ColorTrigger {
    pub fn new(target: Color, rgba: (u8, u8, u8, f32), duration: f32, blending: bool) -> Obj {
        Obj::raw(899).set(23, target.0).set(7, rgba.0).set(8, rgba.1).set(9, rgba.2)
            .set(35, rgba.3).set(10, duration).set_bool(17, blending)
    }
}

pub struct StopTrigger;
pub enum StopMode { Stop, Pause, Resume }
impl StopTrigger {
    pub fn new(target: Group, mode: StopMode) -> Obj {
        let m = match mode { StopMode::Stop => 0, StopMode::Pause => 1, StopMode::Resume => 2 };
        Obj::raw(1616).set(51, target.0).set(580, m)
    }
}

pub struct RandomTrigger;
impl RandomTrigger {
    pub fn new(a: Group, b: Group, chance_a: f32) -> Obj {
        Obj::raw(1912).set(51, a.0).set(71, b.0).set(10, chance_a * 100.0)
    }
}

pub struct CountTrigger;
impl CountTrigger {
    pub fn new(item: Item, target_count: i32, target: Group, activate: bool, multi: bool) -> Obj {
        Obj::raw(1611).set(80, item.0).set(77, target_count).set(51, target.0)
            .set_bool(56, activate).set_bool(87, multi)
    }
}

pub struct PickupTrigger;
impl PickupTrigger {
    pub fn new(item: Item, delta: i32, override_val: bool) -> Obj {
        Obj::raw(1817).set(80, item.0).set(77, delta).set_bool(139, override_val)
    }
}

pub struct ShakeTrigger;
impl ShakeTrigger {
    pub fn new(strength: f32, interval: f32, duration: f32) -> Obj {
        Obj::raw(1520).set(75, strength).set(84, interval).set(10, duration)
    }
}

pub struct CameraZoomTrigger;
impl CameraZoomTrigger {
    pub fn new(zoom: f32, duration: f32, easing: Easing) -> Obj {
        Obj::raw(1913).set(371, zoom).set(10, duration).set(30, ease(easing))
    }
}

pub struct StartPos;
impl StartPos {
    pub fn new(x: f32, y: f32) -> Obj { Obj::raw(31).at(x, y) }
}
