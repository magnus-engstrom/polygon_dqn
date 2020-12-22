use sandbox::ray::Ray;
use core::option::Option::Some;
use geo::{Line, LineString, Point};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::{pixels, rect, EventPump, Sdl};

pub struct Renderer {
    pub canvas: Option<WindowCanvas>,
    pub sdl_context: Sdl,
    pub event_pump: EventPump,
    pub scalex: f64,
    pub scaley: f64,
    pub x: f64,
    pub render: bool,
}

impl Renderer {
    pub fn new(_scalex: f64, _scaley: f64) -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Renderer {
            canvas: None,
            sdl_context,
            event_pump,
            scalex: 500.0,
            scaley: 500.0,
            x: 0.5,
            render: false,
        }
    }
    pub fn init(&mut self) {
        let video_subsystem = self.sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 1000, 1000)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        self.canvas = Some(canvas);
    }
    pub fn quit(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    self.render = !self.render;
                }
                _ => {}
            }
        }
        false
    }

    pub fn clear(&mut self) {
        if self.canvas.is_none() {
            return
        }
        self.canvas.as_mut().unwrap().set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.as_mut().unwrap().clear();
    }

    pub fn render_rays<C: Clone + Into<pixels::Color>>(&mut self, rays: &Vec<Ray>, color: C, center: &Point<f64>) {
        if self.canvas.is_none() {
            return
        }
        if !self.render {
            return;
        }
        for ray in rays {
            self.render_lines(
                &ray.line_string.lines().into_iter().collect(),
                color.clone(),
                center,
            );
        }
    }
    fn offset(&self, center: &Point<f64>) -> (f64, f64) {
        let ax = center.x() * self.scalex * self.x + (self.scalex / 2.0);
        let ay = center.y() * self.scaley * self.x + (self.scaley / 2.0);
        return (-ax, -ay)
    }
    pub fn render_points<C: Into<pixels::Color>>(&mut self, points: &Vec<Point<f64>>, color: C, center: &Point<f64>) {
        if self.canvas.is_none() {
            return
        }
        if !self.render {
            return;
        }
        let (ax, ay) = self.offset(center);
        self.canvas.as_mut().unwrap().set_draw_color(color);
        for point in points {
            self.canvas.as_mut().unwrap()
                .draw_point(
                    rect::Point::new(
                        (ax + point.x() * self.scalex) as i32,
                        (ay + point.y() * self.scaley) as i32,
                    )
                )
                .unwrap();
        }
    }

    pub fn render_lines<C: Into<pixels::Color>>(&mut self, lines: &Vec<Line<f64>>, color: C, center: &Point<f64>) {
        if self.canvas.is_none() {
            return
        }
        if !self.render {
            return;
        }
        let (ax, ay) = self.offset(center);
        self.canvas.as_mut().unwrap().set_draw_color(color);
        for line in lines {
            self.canvas.as_mut().unwrap()
                .draw_line(
                    rect::Point::new(
                        (ax + line.start.x * self.scalex) as i32,
                        (ay + line.start.y * self.scaley) as i32,
                    ),
                    rect::Point::new(
                        (ax + line.end.x * self.scalex) as i32,
                        (ay + line.end.y * self.scaley) as i32,
                    ),
                )
                .unwrap();
        }
    }

    pub fn render_line_strings<C: Into<pixels::Color>>(
        &mut self,
        lines: &Vec<&LineString<f64>>,
        color: C,
        center: &Point<f64>
    ) {
        if self.canvas.is_none() {
            return
        }
        if !self.render {
            return;
        }
        let (ax, ay) = self.offset(center);
        self.canvas.as_mut().unwrap().set_draw_color(color);
        for line in lines {
            for line_segment in line.lines() {
                self.canvas.as_mut().unwrap()
                    .draw_line(
                        rect::Point::new(
                            (ax + line_segment.start.x * self.scalex) as i32,
                            (ay + line_segment.start.y * self.scaley) as i32,
                        ),
                        rect::Point::new(
                            (ax + line_segment.end.x * self.scalex) as i32,
                            (ay + line_segment.end.y * self.scaley) as i32,
                        ),
                    )
                    .unwrap();
            }
        }
    }

    pub fn present(&mut self) {
        if self.canvas.is_none() {
            return
        }
        self.canvas.as_mut().unwrap().present();
    }
}