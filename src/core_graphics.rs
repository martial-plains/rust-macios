//! Harness the power of Quartz technology to perform lightweight 2D rendering
//! with high-fidelity output. Handle path-based drawing, antialiased
//! rendering, gradients, images, color management, PDF documents, and more.

mod type_defs;
pub use type_defs::*;

pub mod traits;

mod cg_function;
pub use cg_function::*;

mod cg_point;
mod cg_rect;
mod cg_size;

pub use cg_point::*;
pub use cg_rect::*;
pub use cg_size::*;

mod cg_geometry;
pub use cg_geometry::*;
