mod geometry;
mod utils;

use std::fmt::format;

use wasm_bindgen::prelude::*;

use crate::geometry::g3d::{Point3, Vector3};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn rotate(&self, angle: f32) -> Point {
        let x = self.x as f32;
        let y = self.y as f32;

        let s = angle.sin();
        let c = angle.cos();

        let x_new = x * c - y * s;
        let y_new = x * s + y * c;

        Point::new(x_new.round() as i32, y_new.round() as i32)
    }

    pub fn shift(&self, vector: Point) -> Point {
        Point::new(self.x + vector.x, self.y + vector.y)
    }
}

#[derive(Default, Debug)]
pub struct Polygon {
    points: Vec<Point>,
    angle: f32,
    center: Point,
    speed_vector: Point,
    speed_angle: f32,
}

impl Polygon {
    pub fn render(&self, buf: &mut Vec<i32>) -> i32 {
        let rotated_points = self.points.iter().map(|&p| p.rotate(self.angle));
        let shifted_points = rotated_points.map(|p| p.shift(self.center));

        let number_of_numbers = 2 * self.points.len() as i32;

        buf.push(number_of_numbers);

        for p in shifted_points {
            buf.push(p.x);
            buf.push(p.y);
        }
        number_of_numbers + 1
    }

    pub fn tick(&mut self) {
        self.angle += self.speed_angle;
        self.center = self.center.shift(self.speed_vector);
    }
}

#[wasm_bindgen]
pub struct Scene {
    scene_tmp: geometry::SceneTmp,
}

#[wasm_bindgen]
impl Scene {
    pub fn new() -> Self {
        let tri1 = geometry::g3d::Triagnle3::new(
            geometry::g3d::Point3::new(200, 200, 0),
            geometry::g3d::Point3::new(600, 200, 0),
            geometry::g3d::Point3::new(300, 500, 0),
        );
        let tri2 = geometry::g3d::Triagnle3::new(
            geometry::g3d::Point3::new(200, 200, 50),
            geometry::g3d::Point3::new(600, 200, 50),
            geometry::g3d::Point3::new(300, 500, 50),
        );

        // let tri1 = geometry::g3d::Triagnle3::new(
        //     geometry::g3d::Point3::new(200, 200, 500),
        //     geometry::g3d::Point3::new(600, 200, 500),
        //     geometry::g3d::Point3::new(300, 500, 500),
        // );
        
        // let tri2 = geometry::g3d::Triagnle3::new(
        //     geometry::g3d::Point3::new(200, 200, 505),
        //     geometry::g3d::Point3::new(600, 200, 505),
        //     geometry::g3d::Point3::new(300, 400, 600),
        // );

        // let tri3 = geometry::g3d::Triagnle3::new(
        //     geometry::g3d::Point3::new(600, 205, 505),
        //     geometry::g3d::Point3::new(305, 500, 505),
        //     geometry::g3d::Point3::new(295, 395, 600),
        // );

        // let tri4 = geometry::g3d::Triagnle3::new(
        //     geometry::g3d::Point3::new(200, 205, 505),
        //     geometry::g3d::Point3::new(295, 500, 505),
        //     geometry::g3d::Point3::new(305, 395, 600),
        // );

        let shapes = geometry::SetOfTriangles::new(
            vec![tri1, tri2],
            Point3::new(0, 0, 0),
            Vector3::new(100, 100, 10),
            0.01,
            Vector3::new(0, 0, 0)
        );
        let scene_tmp = geometry::SceneTmp::new(shapes);
        Self { scene_tmp }
    }

    pub fn tick(&mut self) {
        self.scene_tmp.step();
    }

    pub fn render(&self) -> *const i32 {
        self.scene_tmp.render()
    }
}
