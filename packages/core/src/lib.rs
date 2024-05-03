pub mod cases;
pub mod domains;
pub mod infra;
pub mod shared;
pub mod types;

pub mod prelude {
    pub use crate::infra::*;
    pub use crate::shared::*;
    pub use crate::types::*;
}
