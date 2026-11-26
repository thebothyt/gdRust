mod consts;
mod level;
mod obj;
pub mod objects;
pub mod triggers;

pub use consts::{event, particle};
pub use level::{Level, Settings};
pub use obj::*;

pub mod prelude {
    pub use crate::consts::{event, particle};
    pub use crate::level::{Level, Settings};
    pub use crate::obj::*;
    pub use crate::objects::*;
    pub use crate::triggers::*;
}
