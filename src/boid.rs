use pinwheel::{signal::{Mutable, Signal, SignalExt}, prelude::{Component, svg::polygon}, Node};

use crate::vector::Vec2d;

#[derive(Debug, Default, Clone)]
pub(crate) struct Boid {
	position: Mutable<Vec2d>,
	velocity: Mutable<Vec2d>,
	acceleration: Mutable<Vec2d>
}

impl Boid {

	pub fn new(position: Vec2d, velocity: Vec2d, acceleration: Vec2d) -> Self {
		Self {
			position: Mutable::new(position),
			velocity: Mutable::new(velocity),
			acceleration: Mutable::new(acceleration)
		}
	}
	pub fn new_at(position: Vec2d) -> Self {
		Self {
			position: Mutable::new(position), ..Default::default()
		}
	}

	pub fn next2(&self) {
		self.position.replace_with(|position| *position + self.velocity.get());
		self.velocity.replace_with(|velocity| *velocity + self.acceleration.get());
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
	fn points2(&self) -> impl Signal<Item = String> {
		fn rotate_pt(point: Vec2d, origin: Vec2d, angle: f32) -> Vec2d {
			// gloo_console::log!("angle: ", angle);
			let s = angle.sin();
			let c = angle.cos();

			let mut new = point - origin;
			new.x = (new.x * c) - (new.y * s);
			new.y = (new.x * s) + (new.y * c);

			// translate back
			let ret = new - origin;
			gloo_console::log!("x: ", ret.x, ", y:", ret.y);
			ret
		}
		fn rotate_pt2(point: Vec2d, origin: Vec2d, angle: f32) -> Vec2d {
			let x_shift = point.x - origin.x;
			let y_shift = point.y - origin.y;
			let x = origin.x + (x_shift * angle.cos() - y_shift * angle.sin());
			let y = origin.y + (x_shift * angle.sin() + y_shift * angle.cos());
			Vec2d {
				x,
				y
			}
		}
		let velocity = self.velocity.get_cloned(); // This is a hack, the whole data in boid should be behind a single mutable

		self.position.signal()
			.map(move|position| {
				let points = Self::template();
				points
					.iter()
					.map(move |point| rotate_pt2(*point, Self::origin(),  (velocity.clone()).heading())	) // calculate rotation based on velocity
					.map(|pt| pt * 2.0) // scale
					.map(|point| point + position) // translate horizontally
					.map(|Vec2d{x,y}| {
							format!("{x},{y}")
					})
					.fold(String::with_capacity(points.len() * 4), |mut acc, next| {
						acc.push_str(&next);
						acc.push(' ');
						acc
					})
			})
	}
}

impl Component for Boid {
    fn into_node(self) -> Node {
		polygon()
			.attribute_signal("points", self.points2())
			.attribute("fill", "blue")
			.into_node()
    }
}