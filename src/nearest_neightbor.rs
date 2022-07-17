use crate::{vector::Vec2d, boid::BoidInner};

type Map = fnv::FnvHashMap<Zone, Vec<BoidInner>>;
pub struct NearestNeightborsMap {
    map: Map,
    edge_length: i32
}

impl NearestNeightborsMap {
    pub(crate) fn new(edge_length: i32, boids: impl IntoIterator<Item = BoidInner>) -> Self {
        let map = boids.into_iter()
            .fold(
                Map::default(), 
                |mut map: Map, boid| {
                    map.entry(Zone::new(boid.position, edge_length)).or_default().push(boid);
                    map
                }
            );
            Self {map, edge_length}
    }
    pub (crate) fn find_within_radius(&self, origin: Vec2d, radius: f32) -> impl Iterator<Item=BoidInner> + '_{
        let origin_zone = Zone::new(origin, self.edge_length);
        let zone_radius = (radius/ self.edge_length as f32).ceil() as i32;
        ZoneIter::new(&origin_zone, zone_radius)
            .filter_map(|zone| self.map.get(&zone))
            .flatten()
            .filter(move |boid| boid.position.distance(&origin) < radius && boid.position != origin)
            .copied()
    }
}

/// A zone represents a square in 2d space starting at the origin and having edge lengths of the specified EDGE_LENGTH. 
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
struct Zone {
    x: i32,
    y: i32
}

impl Zone {
    fn new(point: Vec2d, edge_length: i32) -> Self {
        let x = point.x as i32 / edge_length;
        let y = point.y as i32 / edge_length;
        Self {
            x, y
        }
    }
}


struct ZoneIter {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    prior: Option<Zone>
}

impl ZoneIter {
    pub fn new(zone: &Zone, radius: i32) -> Self {
        Self {
            start_x: zone.x -radius,
            start_y: zone.y - radius,
            end_x: zone.x + radius,
            end_y: zone.y + radius,
            prior: None
        }
    }
}
impl Iterator for ZoneIter {
    type Item = Zone;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if let Some(mut prior) = self.prior {
            if prior.x == self.end_x {
                prior.x = self.start_x;
                prior.y += 1;
                if prior.y > self.end_y {
                    return None
                }
            } else {
                prior.x += 1;
            }
            prior
        } else {
           Zone {x: self.start_x, y: self.start_y} 
        };
        self.prior = Some(next);
        Some(next)
    }
}
