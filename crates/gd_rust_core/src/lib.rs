mod level;
mod obj;
pub mod objects;
pub mod triggers;

pub use level::{Level, Settings};
pub use obj::*;

pub mod prelude {
    pub use crate::level::{Level, Settings};
    pub use crate::obj::*;
    pub use crate::objects::*;
    pub use crate::triggers::*;
}
