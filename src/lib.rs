pub mod nbody;
mod utils;

#[cfg(feature = "native")]
use log::info;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "web")]
#[wasm_bindgen]
pub fn logging_init() {
    utils::set_panic_hook();
    wasm_logger::init(wasm_logger::Config::default());
}

// rayon glium demo
#[cfg(feature = "native")]
pub fn run() {
    use std::env;
    env_logger::init();
    info!("Run nbody demo");
    let args: Vec<String> = env::args().collect();
    nbody::main(&args);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "web")]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Seeded Rng
pub fn seeded_rng() -> rand_xorshift::XorShiftRng {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    let mut seed = <XorShiftRng as SeedableRng>::Seed::default();
    (0..).zip(seed.as_mut()).for_each(|(i, x)| *x = i);
    XorShiftRng::from_seed(seed)
}
