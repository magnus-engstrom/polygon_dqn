use geo::{Point, Rect};

use crate::ray::Ray;

pub struct Agent {
    pub speed: f64,
    pub direction: f64,
    pub ray_count: f64,
    pub fov: f64,
    pub visibility: f64,
    pub position: Point<f64>,
    pub rays: Vec<Ray>,
    pub rays_bb: Rect<f64>,
}

impl Agent {
    pub(crate) fn new(position: (f64, f64), direction: f64) -> Self {
        Agent {
            speed: 0.0004,
            direction,
            ray_count: 128.0,
            fov: 0.5,
            visibility: 0.6,
            position: Point::from(position),
            rays: vec![],
            rays_bb:Rect::new((f64::NEG_INFINITY,f64::NEG_INFINITY),(f64::INFINITY,f64::INFINITY))
        }
    }

    pub fn cast_rays(&mut self) {
        self.rays.clear();
        let (rays, rays_bb) = Ray::generate_rays(
            self.ray_count,
            self.fov,
            self.visibility,
            self.direction,
            self.position,
        );
        self.rays = rays;
        self.rays_bb = rays_bb;
    }

    pub fn step(&mut self, direction_change: f64) {
        self.direction += direction_change;
        self.position = Point::new(
            self.position.x() + self.speed * self.direction.cos(),
            self.position.y() + self.speed * self.direction.sin(),
        );
        self.cast_rays();
    }
}
