use crate::obj::*;

pub struct Block;
impl Block {
    pub fn default(x: f32, y: f32) -> Obj { Obj::raw(1).at(x, y) }
    pub fn slab(x: f32, y: f32) -> Obj { Obj::raw(40).at(x, y) }
    pub fn slope(x: f32, y: f32, angle: f32) -> Obj { Obj::raw(315).at(x, y).set(6, angle) }
    pub fn custom(id: u16, x: f32, y: f32) -> Obj { Obj::raw(id).at(x, y) }
}

pub struct Spike;
impl Spike {
    pub fn regular(x: f32, y: f32) -> Obj { Obj::raw(8).at(x, y) }
    pub fn small(x: f32, y: f32) -> Obj { Obj::raw(103).at(x, y) }
    pub fn ground(x: f32, y: f32) -> Obj { Obj::raw(9).at(x, y) }
    pub fn custom(id: u16, x: f32, y: f32) -> Obj { Obj::raw(id).at(x, y) }
}

pub struct Sawblade;
impl Sawblade {
    pub fn new(x: f32, y: f32, radius: f32) -> Obj {
        let s = radius * 2.0; // base saw is ~1 block wide
        Obj::raw(88).at(x, y).with_transform(s, s, 0.0)
    }
}

pub struct Orb;
impl Orb {
    pub fn yellow(x: f32, y: f32) -> Obj { Obj::raw(36).at(x, y) }
    pub fn blue(x: f32, y: f32) -> Obj { Obj::raw(84).at(x, y) }
    pub fn pink(x: f32, y: f32) -> Obj { Obj::raw(141).at(x, y) }
    pub fn green(x: f32, y: f32) -> Obj { Obj::raw(1022).at(x, y) }
    pub fn red(x: f32, y: f32) -> Obj { Obj::raw(1333).at(x, y) }
    pub fn black(x: f32, y: f32) -> Obj { Obj::raw(1330).at(x, y) }
    pub fn spider(x: f32, y: f32) -> Obj { Obj::raw(1594).at(x, y) }
    pub fn dash(x: f32, y: f32, angle: f32) -> Obj { Obj::raw(1704).at(x, y).set(6, angle) }
}

pub struct Pad;
impl Pad {
    pub fn yellow(x: f32, y: f32) -> Obj { Obj::raw(35).at(x, y) }
    pub fn pink(x: f32, y: f32) -> Obj { Obj::raw(140).at(x, y) }
    pub fn blue(x: f32, y: f32) -> Obj { Obj::raw(67).at(x, y) }
    pub fn red(x: f32, y: f32) -> Obj { Obj::raw(1332).at(x, y) }
    pub fn spider(x: f32, y: f32) -> Obj { Obj::raw(2926).at(x, y) }
}

pub struct UserCoin;
impl UserCoin { pub fn new(x: f32, y: f32) -> Obj { Obj::raw(1329).at(x, y) } }

#[derive(Clone, Copy)]
pub enum Mode { Cube, Ship, Ball, Ufo, Wave, Robot, Spider, Swing }
#[derive(Clone, Copy)]
pub enum Speed { Half, Normal, Double, Triple, Quad }

pub struct Portal;
impl Portal {
    pub fn gamemode(x: f32, y: f32, m: Mode) -> Obj {
        let id = match m {
            Mode::Cube => 12, Mode::Ship => 13, Mode::Ball => 47, Mode::Ufo => 111,
            Mode::Wave => 660, Mode::Robot => 745, Mode::Spider => 1331, Mode::Swing => 1933,
        };
        Obj::raw(id).at(x, y)
    }
    pub fn speed(x: f32, y: f32, s: Speed) -> Obj {
        let id = match s { Speed::Half => 200, Speed::Normal => 201, Speed::Double => 202, Speed::Triple => 203, Speed::Quad => 1334 };
        Obj::raw(id).at(x, y)
    }
    pub fn gravity(x: f32, y: f32, inverted: bool) -> Obj { Obj::raw(if inverted { 11 } else { 10 }).at(x, y) }
    pub fn size(x: f32, y: f32, mini: bool) -> Obj { Obj::raw(if mini { 101 } else { 99 }).at(x, y) }
    pub fn dual(x: f32, y: f32, on: bool) -> Obj { Obj::raw(if on { 286 } else { 287 }).at(x, y) }
    pub fn mirror(x: f32, y: f32, on: bool) -> Obj { Obj::raw(if on { 45 } else { 46 }).at(x, y) }
}

pub struct Deco;
impl Deco {
    pub fn custom(id: u16, x: f32, y: f32) -> Obj { Obj::raw(id).at(x, y) }
    pub fn glow(x: f32, y: f32, sx: f32, sy: f32) -> Obj { Obj::raw(1888).at(x, y).with_transform(sx, sy, 0.0) }
    pub fn pixel(x: f32, y: f32, id: u16) -> Obj { Obj::raw(id).at(x, y) }
}
