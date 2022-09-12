use crate::{Point, term};
use termion;
use std::io::Write;
use rand::prelude::*;

const BASE_ENERGY: f32 = 100f32;
const RADIUS: f32 = 1f32;
const DAMP: f32 = 1.0;
const GRAVITY: Point = Point { x: 0f32, y: 1.0 };


#[derive(Copy, Clone, Debug)]
struct Particle {
    /// Location
    l: Point,
    /// Velocity
    v: Point,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            l: Point::origin(),
            v: Point::origin(),
        }
    }
}

impl Particle {
    fn new(x: f32, y: f32, dx: f32, dy: f32) -> Particle {
        Particle {
            l: Point{ x, y },
            v: Point{ x: dx, y: dy },
        }
    }

    fn colliding(&self, rhs: &Particle) -> bool {
        self.l.distance_sq(&rhs.l) <= (RADIUS * 2f32).powi(2)
    }

    /// Collide the two points, modify their
    fn collide(&mut self, rhs: &mut Particle) {
        debug_assert!(self.colliding(rhs));
        let v1v2 = self.v - rhs.v;
        let c1c2 = self.l - rhs.l;
        let v2v1 = rhs.v - self.v;
        let c2c1 = rhs.l - self.l;
        self.v = self.v - (c1c2 * (v1v2.inner(c1c2) / c1c2.magnitude().powi(2)));
        rhs.v  = rhs.v  - (c2c1 * (v2v1.inner(c2c1) / c2c1.magnitude().powi(2)));
    }

    fn gravity(&mut self, dt: f32) {
        self.v += GRAVITY * dt;
    }

    fn tick(&mut self, dt: f32) {
        self.l += self.v * dt
    }
}

pub struct Universe {
    particles: Vec<Particle>,
    width: u16,
    height: u16,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            particles: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn frame<W: Write>(&mut self, stdout: &mut termion::raw::RawTerminal<W>) {
        self.update_dims();
        self.tick();
        self.render(stdout);
    }

    pub fn add_rand(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>() * self.width as f32;
        let y = rng.gen::<f32>() * self.height as f32;
        let dx = (rng.gen::<f32>() - 0.5) * BASE_ENERGY;
        let dy = (rng.gen::<f32>() - 0.5) * BASE_ENERGY;
        self.add(Particle::new(x, y, dx, dy));
    }

    fn term_width(&self) -> usize {
        (self.width / 2) as usize
    }

    fn term_height(&self) -> usize {
        (self.height / 4) as usize
    }

    fn render<W: Write>(&self, stdout: &mut termion::raw::RawTerminal<W>) {
        let mut screen = term::Screen::new(self.term_width(), self.term_height());
        self.particles.iter().for_each(|p| screen.add(&p.l));
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
            screen.to_string(),
        ).unwrap();
        stdout.flush().unwrap();
    }

    pub fn tick_many(&mut self, n: usize) {
        for _ in 0..n {
            self.tick()
        }
    }

    /// Simulate physics 1 tick
    pub fn tick(&mut self) {
        let dt = 0.01;

        for p in self.particles.iter_mut() {
            p.tick(dt);
            //p.gravity(dt);
        }
        self.check_walls();
        self.check_collisions();
    }

    /// Check and apply collisions to all points
    fn check_collisions(&mut self) {
        let l = self.particles.len();
        for i in 0..l {
            for j in i+1..l {
                let (l, r) = self.particles.split_at_mut(j);
                let a = &mut l[i];
                let b = &mut r[0];
                if a.colliding(b) {
                    a.collide(b);
                }
            }
        }
    }

    fn check_walls(&mut self) {
        for p in self.particles.iter_mut() {
            let x = p.l.x as u16;
            let y = p.l.y as u16;

            if x <= 0 {
                p.l.x *= -1f32;
                p.v.x *= -1f32 * DAMP;
            } else if x >= self.width {
                p.l.x = 2f32 * (self.width as f32) - p.l.x;
                p.v.x *= -1f32 * DAMP;
            }

            if y <= 0 {
                p.l.y *= -1f32;
                p.v.y *= -1f32 * DAMP;
            } else if y >= self.height {
                p.l.y = 2f32 * (self.height as f32) - p.l.y;
                p.v.y *= -1f32 * DAMP;
            }
        }
    }

    /// Update interal dimensions, pushing particles as needed
    pub fn update_dims(&mut self) {
        let (mut w, mut h) = term::dims();
        w *= 2;
        h *= 4;
        if w != self.width {
            self.set_width(w);
        }
        if h != self.height {
            self.set_height(h);
        }
    }

    fn add(&mut self, p: Particle) {
        self.particles.push(p)
    }

    /// Update the width, moving out of bounds particles
    fn set_width(&mut self, width: u16) {
        self.width = width;
        self.check_walls();
    }

    /// Update the height, moving out of bounds particles
    fn set_height(&mut self, height: u16) {
        self.height = height;
        self.check_walls();
    }
}

