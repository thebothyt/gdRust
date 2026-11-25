use crate::obj::*;
use crate::objects::{Mode, Speed};
use base64::{engine::general_purpose::URL_SAFE, Engine};
use flate2::{write::GzEncoder, Compression};
use std::io::Write;

pub struct Settings {
    pub gamemode: Mode,
    pub speed: Speed,
    pub mini: bool,
    pub platformer: bool,
    pub song_id: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { gamemode: Mode::Cube, speed: Speed::Normal, mini: false, platformer: false, song_id: 0 }
    }
}

pub struct Level {
    pub name: String,
    pub author: String,
    pub settings: Settings,
    objects: Vec<Obj>,
    group: u16,
    color: u16,
    item: u16,
    block: u16,
    remap: u16,
}

impl Level {
    pub fn new(name: &str) -> Self {
        Level {
            name: name.into(), author: "gdrust".into(), settings: Settings::default(),
            objects: vec![], group: 0, color: 0, item: 0, block: 0, remap: 10000,
        }
    }

    pub fn add(&mut self, o: Obj) -> &mut Self { self.objects.push(o); self }
    pub fn add_all(&mut self, os: impl IntoIterator<Item = Obj>) -> &mut Self { self.objects.extend(os); self }

    pub fn next_group(&mut self) -> Group { self.group += 1; Group(self.group) }
    pub fn next_color(&mut self) -> Color { self.color += 1; Color(self.color) }
    pub fn next_item(&mut self) -> Item { self.item += 1; Item(self.item) }
    pub fn next_block_id(&mut self) -> BlockId { self.block += 1; BlockId(self.block) }
    // remap ids live up high so they dont fight real groups
    pub fn next_remap_id(&mut self) -> u16 { self.remap += 1; self.remap }

    fn header(&self) -> String {
        let s = &self.settings;
        let mode = match s.gamemode {
            Mode::Cube => 0, Mode::Ship => 1, Mode::Ball => 2, Mode::Ufo => 3,
            Mode::Wave => 4, Mode::Robot => 5, Mode::Spider => 6, Mode::Swing => 7,
        };
        let speed = match s.speed { Speed::Half => 1, Speed::Normal => 0, Speed::Double => 2, Speed::Triple => 3, Speed::Quad => 4 };
        format!(
            "kA13,0,kA15,0,kA16,0,kA14,,kA6,0,kA7,0,kA17,0,kA18,0,kS39,0,kA2,{mode},kA3,{},kA8,0,kA4,{speed},kA9,0,kA10,0,kA22,0,kA23,0,kA24,0,kA27,1,kA40,1,kA41,1,kA42,1,kA28,0,kA29,0,kA31,1,kA32,1,kA36,0,kA43,0,kA44,0,kA45,1,kA46,0,kA33,1,kA34,1,kA35,0,kA37,1,kA38,1,kA39,1,kA19,0,kA26,0,kA20,0,kA21,0,kA11,0,kA21,{};",
            s.mini as u8, s.platformer as u8
        )
    }

    pub fn raw_level_string(&self) -> String {
        let mut out = self.header();
        for o in &self.objects { out.push_str(&o.serialize()); out.push(';'); }
        out
    }

    fn encoded(&self) -> String {
        let mut gz = GzEncoder::new(Vec::new(), Compression::default());
        gz.write_all(self.raw_level_string().as_bytes()).unwrap();
        URL_SAFE.encode(gz.finish().unwrap())
    }

    // wasm-safe, hand this straight to a browser download
    pub fn to_gmd_string(&self) -> String {
        format!(
            "<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\"><dict>\
<k>k1</k><i>1</i><k>k2</k><s>{}</s><k>k3</k><s></s><k>k4</k><s>{}</s>\
<k>k5</k><s>{}</s><k>k13</k><t/><k>k21</k><i>2</i><k>k50</k><i>45</i></dict></plist>",
            self.name, self.encoded(), self.author
        )
    }

    #[cfg(feature = "fs")]
    pub fn export_gmd(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_gmd_string())
    }
}
