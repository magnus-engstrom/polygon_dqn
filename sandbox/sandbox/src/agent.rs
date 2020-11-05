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
    pub age: f64,
    pub targets_count: i32,
    pub closest_target: Point<f64>,
    pub max_age: f64,
}

impl Agent {
    pub(crate) fn new(position: Point<f64>, direction: f64) -> Self {
        Agent {
            speed: 0.005,
            age: 1.0,
            direction,
            ray_count: 21.0,
            fov: 2.0,
            visibility: 0.3,
            max_age: 500.0,
            position: position,
            rays: vec![],
            rays_bb:Rect::new((f64::NEG_INFINITY,f64::NEG_INFINITY),(f64::INFINITY,f64::INFINITY)),
            targets_count: 0,
            closest_target: Point::new(0.0,0.0),
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

    pub fn step(&mut self, direction_change: f64, make_move: bool) {
        let mut step_size = 0.0;
        if make_move {
            step_size = self.speed;
        }
        self.age = self.age + 1.0;
        self.direction += direction_change;
        if self.direction > 3.14 {
            self.direction = self.direction - 6.28;
        }
        if self.direction < -3.14 {
            self.direction = self.direction + 6.28;
        }
        self.position = Point::new(
            self.position.x() + step_size * self.direction.cos(),
            self.position.y() + step_size * self.direction.sin(),
        );
        self.cast_rays();
    }
}
