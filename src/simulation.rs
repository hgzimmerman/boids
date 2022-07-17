use futures::StreamExt;
use pinwheel::Node;
use pinwheel::prelude::{Component, div};
use pinwheel::prelude::svg::rect;
use pinwheel::signal_vec::MutableVec;
use web_sys::window;

use crate::boid::Boid;
use crate::config::Config;
use crate::nearest_neightbor::{NearestNeightborsMap};
use crate::vector::Vec2d;
pub(crate) struct Simulation {
	boids: MutableVec<Boid>,
    window_dimensions: Vec2d,
}

impl Simulation {

    #[allow(unused)]
	pub fn new() -> Simulation {
        let y = window().unwrap().inner_height().unwrap().as_f64().expect("could not get inner_height") as f32;
        let x = window().unwrap().inner_width().unwrap().as_f64().expect("could not get inner_width") as f32;
        let window_dimensions = Vec2d {x,y};
		Simulation {
			boids: MutableVec::new_with_values(vec![
				Boid::new(Vec2d{x: 10.0, y:10.0}, Vec2d { x: 1.0, y: 2.0 }, Vec2d::default()),
				Boid::new_at(Vec2d{x: 10.0, y:100.0}),
				Boid::new_at(Vec2d{x: 100.0, y:10.0})
			]),
            window_dimensions
		}

	}
    pub fn new_generate_random(count: usize, ) -> Self{
        let y = window().unwrap().inner_height().unwrap().as_f64().expect("could not get inner_height") as f32;
        let x = window().unwrap().inner_width().unwrap().as_f64().expect("could not get inner_width") as f32;
        let window_dimensions = Vec2d {x,y};
        let vec = (0..count).map(|_| {
            Boid::new(
                Vec2d { x: rand::random::<f32>() * window_dimensions.x, y:  rand::random::<f32>() * window_dimensions.y},
                Vec2d { x: (rand::random::<f32>()-0.5) * 4.0, y: (rand::random::<f32>()-0.5) * 4.0},
                Vec2d::default()
            )
        }).collect();
		Simulation {
			boids: MutableVec::new_with_values(vec),
            window_dimensions
		}
    }
}

impl Component for Simulation {
	fn into_node(self) -> Node {
		let sv = self.boids.signal_vec_cloned();

        let config = Config {
            coherence: 0.25,
            separation: 45.0,
            alignment: 2.0,
            visual_range: 25.0,
            max_speed: 2.8,
            max_acceleration: 0.4
        };

		let on_next = {
			move || {
                // let mut positions = self.boids.lock_ref().iter().map(Boid::position).collect::<Vec<_>>();
                let neighbors = NearestNeightborsMap::new(
                    (config.visual_range / 2.0).ceil() as i32, 
                    self.boids.lock_ref().iter().map(Boid::inner)
                );
				self.boids.lock_ref().iter().for_each(|boid| {
					boid.next(&neighbors, &config, self.window_dimensions);
				});
			}
		};

		wasm_bindgen_futures::spawn_local(async move {
			gloo_timers::future::IntervalStream::new(16).for_each(|_| async {
				on_next()
			}).await;
		});
		
		

		let svg = pinwheel::elements::svg::svg()
			.attribute("height", "100%")
			.attribute("width", "100%")
			.child(
				rect()
				.attribute("height", "100%")
				.attribute("width", "100%")
				.attribute("fill", "red")
			)
			.child_signal_vec(sv)
		;


		div()
			.attribute("style", "width:100%;height:100%;")
			.child(svg)
			.into_node()
	}
}

