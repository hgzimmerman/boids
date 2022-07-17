use pinwheel::{signal::{Mutable, Signal, SignalExt}, prelude::{Component, svg::polygon}, Node};

use crate::{vector::Vec2d, nearest_neightbor::{NearestNeightborsMap}};
use crate::config::Config;

#[derive(Debug, Default, Clone)]
pub(crate) struct Boid {
    inner: Mutable<BoidInner>
}
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct BoidInner {
    pub position: Vec2d,
	pub velocity: Vec2d,
	pub acceleration: Vec2d
}

#[derive(Debug, Default)]
struct Eval {
    /// How many nearby entities.
    count: usize,
    /// Sum of the positions of the nearby boids.
    position_sum: Vec2d,
    /// Sum of the velocites of the nearby boids.
    velocity_sum: Vec2d,
    /// Sum of the avoidance calculations of nearby boids.
    avoidance_sum: Vec2d 
}

impl Boid {
	pub fn new(position: Vec2d, velocity: Vec2d, acceleration: Vec2d) -> Self {
		Self {
            inner: Mutable::new(BoidInner { position, velocity, acceleration })
		}
	}
    #[allow(unused)]
	pub fn new_at(position: Vec2d) -> Self {
		Self {
            inner: Mutable::new(BoidInner { position, ..Default::default() })
		}
	}
    
    pub fn inner(&self) -> BoidInner {
        self.inner.get()
    }

    pub fn next(&self, other_boids: &NearestNeightborsMap, config: &Config, perimeter: Vec2d) {
        self.inner.replace_with(|boid| {
            let Eval { count, position_sum, velocity_sum, avoidance_sum } =other_boids
                .find_within_radius(boid.position, config.visual_range)
                .fold(Eval::default(), |mut acc, next| {
                    acc.count += 1;
                    acc.position_sum += next.position;
                    acc.velocity_sum += next.velocity;
                    acc.avoidance_sum += {
                        let vector_to_boid = boid.position - next.position;
                        let distance = vector_to_boid.length();
                        // more power the closer they are
                        let power = 1.0 / distance;
                        let normalized = vector_to_boid.normalize();
                        normalized * power * config.separation
                    };
                    acc
                });

            BoidInner {
                position: boid.position + boid.velocity,
                velocity: (boid.velocity + boid.acceleration).clamp(config.max_speed) * (std::cmp::max(12, count) as f32 / 12.0),
                acceleration: {
                    let mut acceleration = if count == 0 {
                        boid.acceleration
                    } else {
                        let coherence_acceleration = -(boid.position - position_sum / count as f32) * config.coherence;
                        let alignment_acceleration = (velocity_sum / count as f32) * config.alignment;
                        let avoidance_acceleration = avoidance_sum;
                        coherence_acceleration + alignment_acceleration + avoidance_acceleration
                    };
                    

                    const PERIMETER_FRAME: f32 = 50.0;
                    const RETURN_MULTIPLIER: f32 = 2.0;

                    if boid.position.x < 0.0 + PERIMETER_FRAME {
                        acceleration.x = acceleration.x.abs() * RETURN_MULTIPLIER + 0.1
                    }
                    if boid.position.y < 0.0 + PERIMETER_FRAME{
                        acceleration.y = acceleration.y.abs() * RETURN_MULTIPLIER + 0.1
                    }
                    if boid.position.x > perimeter.x - PERIMETER_FRAME {
                        acceleration.x = -acceleration.x.abs() * RETURN_MULTIPLIER -0.1
                    }
                    if boid.position.y > perimeter.y - PERIMETER_FRAME {
                        acceleration.y = -acceleration.y.abs() * RETURN_MULTIPLIER -0.1
                    }

                    acceleration.clamp(config.max_acceleration)
                },
            }
        });
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
			let s = angle.sin();
			let c = angle.cos();
			let x_shift = point.x - origin.x;
			let y_shift = point.y - origin.y;
			let x = origin.x + (x_shift * c - y_shift * s);
			let y = origin.y + (x_shift * s + y_shift * c);
			Vec2d {
				x,
				y
			}
		}
        self.inner
            .signal()
			.map(move|boid| {
                let BoidInner {
                    position,
                    velocity,
                    acceleration: _,
                } = boid;
				let points = Self::template();
				points
					.iter()
					.map(move |point| rotate_pt(*point, Self::origin(),  velocity.heading())	) // calculate rotation based on velocity
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