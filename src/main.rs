use config::Config;
use pinwheel::prelude::App;
use web_sys as dom;

mod boid;
mod simulation;
mod vector;
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
	let config = Config {
		coherence: 0.25,
		separation: 45.0,
		alignment: 2.0,
		visual_range: 25.0,
		max_speed: 2.8,
		max_acceleration: 0.4,
	};
    let simulation = Simulation::new_generate_random(800, config);
    App::new(body, simulation).forget();
}
