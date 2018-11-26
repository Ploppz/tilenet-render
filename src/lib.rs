#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate glium;
extern crate tilenet;

pub mod renderer;
pub use renderer::Renderer as Ren;
pub use renderer::MinifySamplerFilter as MinifySamplerFilter;
pub use renderer::MagnifySamplerFilter as MagnifySamplerFilter;
