#[cfg(feature = "native")]
use rayon_nbody_demo;

fn main() {
    #[cfg(feature = "native")]
    rayon_nbody_demo::run();
}
