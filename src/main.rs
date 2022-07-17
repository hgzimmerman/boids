use pinwheel::prelude::App;
use web_sys as dom;

mod vector;
mod boid;
mod simulation;
use simulation::Simulation;
mod config;
mod nearest_neightbor;

fn main() {
	let body = dom::window()
		.unwrap()
		.document()
		.unwrap()
		.body()
		.unwrap()
		.into();
	let simulation = Simulation::new_generate_random(800);
	App::new(body, simulation).forget();
}
