use std::ops::{Add, Sub, Mul};

use pinwheel::prelude::{*, svg::{rect, circle, polygon}};
use web_sys as dom;

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

struct Simulation {
	boids: MutableVec<Boid>
}

impl Simulation {
	pub fn new() -> Simulation {
		Simulation {
			boids: MutableVec::new_with_values(vec![
				Boid::new(Vec2d{x: 10.0, y:10.0}, Vec2d { x: 1.0, y: 0.0 }, Vec2d::default()),
				Boid::new_at(Vec2d{x: 10.0, y:100.0}),
				Boid::new_at(Vec2d{x: 100.0, y:10.0})
			]),
		}
	}
}

impl Component for Simulation {
	fn into_node(self) -> Node {
		// let on_click = {
		// 	let count = self.count.clone();
		// 	move |_| {
		// 		count.replace_with(|count| *count + 1);
		// 	}
		// };

		// let count_text = self
		// 	.count
		// 	.signal()
		// 	.map(|count| p().child(count.to_string()));
		gloo_console::log!("first pass");
		let sv = self.boids.signal_vec();

		let on_next = {
			move |_| {
				gloo_console::log!("calculating next");
				let _ = self.boids.signal_vec_cloned().map(|pt| pt.next());
			}
		};
		let increment_button = button().onclick(on_next).child("Increment");
		

		let svg = pinwheel::elements::svg::svg()
			.attribute("height", "100%")
			.attribute("width", "100%")
			.child(
				rect()
				.attribute("height", "100%")
				.attribute("width", "100%")
				.attribute("fill", "red")
			)
			// .children(self.boids)
			.child_signal_vec(sv)
		;


		div()
			// .onclick(on_next)
			.attribute("style", "width:100%;height:100%;")
			// .child_signal(count_text)
			.child(increment_button)
			// .child(div().child(canvas))
			.child(svg)
			.into_node()
	}
}
#[derive(Debug, Default, Clone, Copy)]
struct Vec2d {
	x: f32,
	y: f32,
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
    }
}
impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
    }
}

impl Mul<f32> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs
		}
    }
}

impl Vec2d {
	// radians
	fn heading(&self) -> f32 {
		f32::atan2(self.x, self.y)
	}
}

#[derive(Debug, Default, Clone, Copy)]
struct Boid {
	position: Vec2d,
	velocity: Vec2d,
	acceleration: Vec2d
}

impl Boid {

	fn new(position: Vec2d, velocity: Vec2d, acceleration: Vec2d) -> Self {
		Self {
			position, velocity, acceleration
		}
	}
	fn new_at(position: Vec2d) -> Self {
		Self {
			position, ..Default::default()
		}
	}

	fn next(self) -> Self {
		Self {
			position: self.position + self.velocity,
			velocity: self.velocity + self.acceleration,
			acceleration: self.acceleration // no jerk yet
		}
	}

	fn template() -> &'static [Vec2d] {
		&[
			Vec2d {x: 0.0, y:10.0},
			Vec2d {x: 3.0, y:0.0},
			Vec2d {x: 6.0, y:10.0},
		]
	}
	fn origin() -> Vec2d {
		Vec2d { x: 3.0, y: 5.0 }
	}
	fn points(&self) -> Vec<Vec2d> {
		fn rotate_pt(point: Vec2d, origin: Vec2d, angle: f32) -> Vec2d {
			let s = angle.sin();
			let c = angle.cos();

			let mut new = point - origin;
			new.x = new.x * c - new.y * s;
			new.y = new.x * s + new.y * c;

			// translate back
			new + origin
		}

		Self::template()
		.iter()
		.map(|pt| Vec2d::mul(*pt,2.0))
		// .map(|point| rotate_pt(point, Self::origin(), self.velocity.heading())	) // calculate rotation based on velocity
		.map(|point| point + self.position) // translate horizontally
		.collect()
	}
}

impl Component for Boid {
    fn into_node(self) -> Node {
		let points = self.points();

		polygon()
			.attribute(
				"points", 
				 points
					.iter()
					.map(|Vec2d{x,y}| {
						format!("{x},{y}")
					})
					.fold(String::with_capacity(points.len() * 4), |mut acc, next| {
						acc.push_str(&next);
						acc.push(' ');
						acc
					})
			)
			.attribute("fill", "blue")
			.into_node()
    }
}