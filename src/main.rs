use pinwheel::prelude::App;
use web_sys as dom;

mod vector;
mod boid;
mod simulation;
use simulation::Simulation;

fn main() {
	let body = dom::window()
		.unwrap()
		.document()
		.unwrap()
		.body()
		.unwrap()
		.into();
	let simulation = Simulation::new();
	App::new(body, simulation).forget();
}
