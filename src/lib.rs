mod utils;

use std::fmt::format;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    _x: i32,
    _y: i32
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: i32, y: i32) -> Point{
        Point {_x: x, _y: y}
    }

    pub fn x(&self) -> i32 {
        self._x
    }

    pub fn y(&self) -> i32 {
        self._y
    }

    pub fn rotate(&mut self, center: Point, angle: f32) {
        let x = (self._x  - center._x) as f32;
        let y = (self._y - center._y) as f32;

        let s = angle.sin();
        let c = angle.cos();


        let x_new = (x*c - y*s) as i32;
        let y_new = (x*s + y*c) as i32;

        self._x = x_new + center._x;
        self._y = y_new + center._y;

    }
}



#[wasm_bindgen]
pub fn start() -> Point {
    Point {_x:0, _y:0}
}

#[wasm_bindgen]
pub fn end() -> Point {
    Point {_x:200, _y:200}
}

#[wasm_bindgen]
pub struct Rect {
    points: Vec<Point>
}

#[wasm_bindgen]
impl Rect {
    pub fn new() -> Rect{
        Rect {points: Vec::new()}
    }
    
    pub fn push(&mut self, point: Point) {
        self.points.push(point)
    }

    pub fn rotate(&mut self, center: Point, angle: f32) {
        for point in self.points.iter_mut() {
            point.rotate(center, angle);
        }
    }

    pub fn points(&self) -> *const Point {
        self.points.as_ptr()
    }

    pub fn size(&self) -> u32 {
        self.points.len() as u32
    }

}

#[wasm_bindgen]
pub struct Scene {
    rects: Vec<Rect>
}

