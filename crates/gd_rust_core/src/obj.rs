use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Group(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Item(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BlockId(pub u16);

#[derive(Clone, Copy, Debug)]
pub struct GridPos(pub f32, pub f32);

#[derive(Clone, Copy, Debug)]
pub enum Easing {
    None, EaseInOut, EaseIn, EaseOut, ElasticInOut, ElasticIn, ElasticOut,
    BounceInOut, BounceIn, BounceOut, ExpoInOut, ExpoIn, ExpoOut,
    SineInOut, SineIn, SineOut,
}

#[derive(Clone, Copy, Debug)]
pub enum ZLayer { B5 = -5, B4 = -3, B3 = -1, B2 = 1, B1 = 3, T1 = 5, T2 = 7, T3 = 9, T4 = 11 }

#[derive(Clone, Copy, Debug)]
pub enum Flag { NoTouch, DontFade, DontEnter, HighDetail, NoEffects, NoGlow }

#[derive(Clone, Debug)]
pub struct Obj {
    pub(crate) props: BTreeMap<u16, String>,
}

impl Obj {
    pub fn raw(id: u16) -> Self {
        let mut props = BTreeMap::new();
        props.insert(1, id.to_string());
        Obj { props }
    }

    pub fn set(mut self, key: u16, val: impl ToString) -> Self {
        self.props.insert(key, val.to_string());
        self
    }

    pub fn set_bool(self, key: u16, v: bool) -> Self {
        if v { self.set(key, 1) } else { self.set(key, 0) }
    }

    // cell (0,0) center sits at 15,15 internally, so +15 keeps blocks grid-aligned
    pub fn at(self, x: f32, y: f32) -> Self {
        self.set(2, x * 30.0 + 15.0).set(3, y * 30.0 + 15.0)
    }
    pub fn at_pos(self, p: GridPos) -> Self { self.at(p.0, p.1) }

    pub fn spawn_triggered(self, multi: bool) -> Self {
        self.set(62, 1).set_bool(87, multi)
    }
    pub fn multi_triggered(self, v: bool) -> Self { self.set_bool(87, v) }
    pub fn touch_triggered(self, v: bool) -> Self { self.set_bool(11, v) }

    pub fn add_groups(mut self, groups: &[Group]) -> Self {
        let mut ids: Vec<String> = self.props.get(&57).map(|s| s.split('.').map(String::from).collect()).unwrap_or_default();
        ids.extend(groups.iter().map(|g| g.0.to_string()));
        self.props.insert(57, ids.join("."));
        self
    }
    pub fn set_parent(self, g: Group) -> Self { self.set(34, 1).add_groups(&[g]) }

    pub fn with_color(self, c: Color) -> Self { self.set(21, c.0) }
    pub fn with_detail_color(self, c: Color) -> Self { self.set(22, c.0) }
    pub fn with_z(self, layer: ZLayer, order: i32) -> Self { self.set(24, layer as i32).set(25, order) }
    pub fn with_transform(self, sx: f32, sy: f32, rot: f32) -> Self {
        self.set(32, sx).set(128, sx).set(129, sy).set(6, rot)
    }
    pub fn with_hsv(self, h: i32, s: f32, b: f32, s_chk: bool, b_chk: bool) -> Self {
        self.set(41, 1).set(43, format!("{h}a{s}a{b}a{}a{}", s_chk as u8, b_chk as u8))
    }
    pub fn set_editor_layer(self, l1: i32, l2: i32) -> Self { self.set(20, l1).set(61, l2) }
    pub fn set_flags(mut self, flags: &[Flag]) -> Self {
        for f in flags {
            let k = match f {
                Flag::NoTouch => 121, Flag::DontFade => 64, Flag::DontEnter => 67,
                Flag::HighDetail => 103, Flag::NoEffects => 116, Flag::NoGlow => 96,
            };
            self.props.insert(k, "1".into());
        }
        self
    }

    pub(crate) fn serialize(&self) -> String {
        self.props.iter().map(|(k, v)| format!("{k},{v}")).collect::<Vec<_>>().join(",")
    }
}
