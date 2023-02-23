pub use super::nbody::NBodyBenchmark;
pub use super::ExecutionMode;
use cgmath::Point3;
use log::info;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[cfg(feature = "web-parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Instance {
    color: [f32; 3],
    world_position: [f32; 3],
}

#[wasm_bindgen]
pub struct NBody {
    num_bodies: usize,
    benchmark: NBodyBenchmark,
    mode: ExecutionMode,
}

#[cfg(feature = "web-parallel")]
#[wasm_bindgen]
impl NBody {
    #[wasm_bindgen(constructor)]
    pub fn new(num_bodies: usize, mode: ExecutionMode) -> NBody {
        let num_bodies = num_bodies; //1500; //

        let benchmark = NBodyBenchmark::new(num_bodies, &mut rand::thread_rng());
        let mode = mode;

        info!("Mode : {:?}", mode);

        NBody {
            num_bodies,
            benchmark,
            mode,
        }
    }

    pub fn init_conditions(&self) -> JsValue {
        let mut rng = crate::seeded_rng();
        let instances: Vec<_> = (0..self.num_bodies)
            .map(|_| Instance {
                color: [
                    rng.gen_range(0.5..1.0),
                    rng.gen_range(0.5..1.0),
                    rng.gen_range(0.5..1.0),
                ],
                world_position: [0.0, 0.0, 0.0],
            })
            .collect();
        JsValue::from_serde(&instances).unwrap()
    }

    pub fn next_positions(&mut self) -> JsValue {
        let bodies = match &self.mode {
            ExecutionMode::Par => self.benchmark.tick_par(),
            ExecutionMode::ParReduce => self.benchmark.tick_par_reduce(),
            ExecutionMode::Seq => self.benchmark.tick_seq(),
        };

        let positions = bodies
            .iter()
            .map(|body| body.position)
            .collect::<Vec<Point3<f64>>>();

        JsValue::from_serde(&positions).unwrap()
    }
}

#[cfg(not(feature = "web-parallel"))]
#[wasm_bindgen]
impl NBody {
    #[wasm_bindgen(constructor)]
    pub fn new(num_bodies: usize, _mode: ExecutionMode) -> NBody {
        let num_bodies = num_bodies; //1500; //

        let benchmark = NBodyBenchmark::new(num_bodies, &mut rand::thread_rng());
        let mode = ExecutionMode::Seq;

        info!("Mode : {:?}", mode);

        NBody {
            num_bodies,
            benchmark,
            mode,
        }
    }

    pub fn init_conditions(&self) -> JsValue {
        let mut rng = crate::seeded_rng();
        let instances: Vec<_> = (0..self.num_bodies)
            .map(|_| Instance {
                color: [
                    rng.gen_range(0.5..1.0),
                    rng.gen_range(0.5..1.0),
                    rng.gen_range(0.5..1.0),
                ],
                world_position: [0.0, 0.0, 0.0],
            })
            .collect();
        JsValue::from_serde(&instances).unwrap()
    }

    pub fn next_positions(&mut self) -> JsValue {
        let bodies = self.benchmark.tick_seq();

        let positions = bodies
            .iter()
            .map(|body| body.position)
            .collect::<Vec<Point3<f64>>>();

        JsValue::from_serde(&positions).unwrap()
    }
}
