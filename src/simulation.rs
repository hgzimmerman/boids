use futures::StreamExt;
use pinwheel::Node;
use pinwheel::prelude::{Component, div};
use pinwheel::prelude::svg::rect;
use pinwheel::signal_vec::MutableVec;

use crate::boid::Boid;
use crate::vector::Vec2d;
pub(crate) struct Simulation {
	boids: MutableVec<Boid>
}

impl Simulation {
	pub fn new() -> Simulation {
		Simulation {
			boids: MutableVec::new_with_values(vec![
				Boid::new(Vec2d{x: 10.0, y:10.0}, Vec2d { x: 1.0, y: 2.0 }, Vec2d::default()),
				Boid::new_at(Vec2d{x: 10.0, y:100.0}),
				Boid::new_at(Vec2d{x: 100.0, y:10.0})
			]),
		}
	}
}

impl Component for Simulation {
	fn into_node(self) -> Node {
		let sv = self.boids.signal_vec_cloned();

		let on_next = {
			move || {
				self.boids.lock_ref().iter().for_each(|boid| {
					boid.next2();
				});
			}
		};

		let _timer_promise = wasm_bindgen_futures::spawn_local(async move {
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

