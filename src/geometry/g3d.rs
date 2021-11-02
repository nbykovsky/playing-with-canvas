use super::g2d;
use crate::geometry::log;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

pub type Vector3 = Point3;

impl Point3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point3 { x, y, z }
    }

    pub fn project(&self) -> g2d::Point2 {
        g2d::Point2::new(self.x, self.y)
    }

    fn add(&self, other: &Point3) -> Point3 {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn neg(&self) -> Point3 {
        Point3::new(-self.x, -self.y, -self.z)
    }

    pub fn shift(&self, scalar: i32) -> Point3 {
        Point3::new(self.x + scalar, self.y + scalar, self.z + scalar)
    }

    /// https://www.khanacademy.org/math/multivariable-calculus/thinking-about-multivariable-function/x786f2022:vectors-and-matrices/a/cross-products-mvc
    fn x_product(&self, other: &Point3) -> Point3 {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Point3::new(x, y, z)
    }

    fn dot_product(&self, other: &Point3) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// https://stackoverflow.com/questions/6721544/circular-rotation-around-an-arbitrary-axis
    fn rotate(&self, axis_point: &Point3, axis_vector: &Vector3, angle: f32) -> Point3 {
        let sin = angle.sin();
        let cos = angle.cos();
        let (x, y, z) = (self.x as f32, self.y as f32, self.z as f32);
        let (a, b, c) = (
            axis_point.x as f32,
            axis_point.y as f32,
            axis_point.z as f32,
        );
        let (u, v, w) = (
            axis_vector.x as f32,
            axis_vector.y as f32,
            axis_vector.z as f32,
        );
        let sm = (u.powf(2.) + v.powf(2.) + w.powf(2.)).sqrt();
        let (u, v, w) = (u / sm, v / sm, w / sm);
        let new_x = (a * (v * v + w * w) - u * (b * v + c * w - u * x - v * y - w * z))
            * (1. - cos)
            + x * cos
            + (-c * v + b * w - w * y + v * z) * sin;
        let new_y = (b * (u * u + w * w) - v * (a * u + c * w - u * x - v * y - w * z))
            * (1. - cos)
            + y * cos
            + (c * u - a * w + w * x - u * z) * sin;
        let new_z = (c * (u * u + v * v) - w * (a * u + b * v - u * x - v * y - w * z))
            * (1. - cos)
            + z * cos
            + (-b * u + a * v - v * x + u * y) * sin;
        Point3::new(
            new_x.round() as i32,
            new_y.round() as i32,
            new_z.round() as i32,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Triagnle3 {
    p1: Point3,
    p2: Point3,
    p3: Point3,
}

impl Triagnle3 {
    pub fn new(p1: Point3, p2: Point3, p3: Point3) -> Self {
        Triagnle3 { p1, p2, p3 }
    }

    /// if projection is a line it will return None
    pub fn project(&self) -> Option<g2d::Triangle2> {
        let p1 = self.p1.project();
        let p2 = self.p2.project();
        let p3 = self.p3.project();

        let on_the_same_line = (p1.y - p2.y) * (p1.x - p3.x) == (p1.y - p3.y) * (p1.x - p2.x);
        if on_the_same_line {
            None
        } else {
            Some(g2d::Triangle2::new(p1, p2, p3))
        }
    }

    /// if true other should be painted BEFORE self
    pub fn is_above(&self, other: &Triagnle3) -> bool {
        match (self.project(), other.project()) {
            (Some(slf), Some(otr)) => {
                if let Some(point) = slf.intersection(&otr) {
                    let self_h = Plane::from_triangle(&self).get_z(point.x, point.y).unwrap();
                    let other_h = Plane::from_triangle(&other)
                        .get_z(point.x, point.y)
                        .unwrap();
                    log(&format!("self_h={};other_h={}", &self_h, &other_h));
                    self_h > other_h
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn rotate(&self, axis_point: &Point3, axis_vector: &Vector3, angle: f32) -> Triagnle3 {
        let p1 = self.p1.rotate(axis_point, axis_vector, angle);
        let p2 = self.p2.rotate(axis_point, axis_vector, angle);
        let p3 = self.p3.rotate(axis_point, axis_vector, angle);
        Triagnle3::new(p1, p2, p3)
    }

    pub fn shift(&self, vector: &Vector3) -> Triagnle3 {
        let p1 = self.p1.add(&vector);
        let p2 = self.p2.add(&vector);
        let p3 = self.p3.add(&vector);
        Triagnle3::new(p1, p2, p3)
    }
}

struct Plane {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Plane {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Plane { a, b, c, d }
    }

    /// https://kitchingroup.cheme.cmu.edu/blog/2015/01/18/Equation-of-a-plane-through-three-points/
    fn from_triangle(triangle: &Triagnle3) -> Self {
        let v1 = triangle.p3.add(&triangle.p1.neg());
        let v2 = triangle.p2.add(&triangle.p1.neg());
        let x_prod = v1.x_product(&v2);
        let dot_prod = x_prod.dot_product(&triangle.p3);
        Plane::new(x_prod.x, x_prod.y, x_prod.z, dot_prod)
    }

    fn get_z(&self, x: i32, y: i32) -> Option<i32> {
        if self.c == 0 {
            return None;
        }
        Some((self.d - x * self.a - y * self.b) / self.c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plane() {
        let triangle = Triagnle3::new(
            Point3::new(0, 0, 0),
            Point3::new(-1, 1, 0),
            Point3::new(1, 1, 0),
        );
        let plane = Plane::from_triangle(&triangle);

        assert_eq!(plane.get_z(0, 0), Some(0));
    }

    #[test]
    fn parallel_triangles() {
        let triangle1 = Triagnle3::new(
            Point3::new(0, 0, 0),
            Point3::new(-1, 1, 0),
            Point3::new(1, 1, 0),
        );
        let triangle2 = Triagnle3::new(
            Point3::new(0, 0, 1),
            Point3::new(-1, 1, 1),
            Point3::new(1, 1, 1),
        );

        assert!(!triangle1.is_above(&triangle2));
        assert!(triangle2.is_above(&triangle1))
    }

    #[test]
    fn overlapping_triangles() {
        let triangle1 = Triagnle3::new(
            Point3::new(0, 0, 0),
            Point3::new(-1, 1, -1),
            Point3::new(1, 1, -1),
        );
        let triangle2 = Triagnle3::new(
            Point3::new(0, 0, 1),
            Point3::new(-1, -1, 2),
            Point3::new(1, -1, 2),
        );

        assert!(!triangle1.is_above(&triangle2));
        assert!(triangle2.is_above(&triangle1));
    }

    #[test]
    fn non_overlapping_triangles() {
        let triangle1 = Triagnle3::new(
            Point3::new(0, 1, 0),
            Point3::new(-1, 2, -1),
            Point3::new(1, 2, -1),
        );
        let triangle2 = Triagnle3::new(
            Point3::new(0, 0, 1),
            Point3::new(-1, -1, 2),
            Point3::new(1, -1, 2),
        );

        assert!(!triangle1.is_above(&triangle2));
        assert!(!triangle2.is_above(&triangle1));
    }

    #[test]
    fn point_rotation() {
        assert_eq!(
            Point3::new(1, 1, 1).rotate(&Point3::new(0, 0, 0), &Vector3::new(1, 1, 1), 10.),
            Point3::new(1, 1, 1)
        );
        assert_eq!(
            Point3::new(1, 0, 0).rotate(
                &Point3::new(0, 0, 0),
                &Vector3::new(0, 0, 1),
                std::f32::consts::PI / 2.
            ),
            Point3::new(0, 1, 0)
        );
        assert_eq!(
            Point3::new(0, 1, 0).rotate(
                &Point3::new(0, 0, 0),
                &Vector3::new(0, 0, 1),
                -std::f32::consts::PI / 2.
            ),
            Point3::new(1, 0, 0)
        );
        assert_eq!(
            Point3::new(1, 1, 0).rotate(
                &Point3::new(0, 0, 0),
                &Vector3::new(-1, 1, 0),
                std::f32::consts::PI / 2.
            ),
            Point3::new(0, 0, -1)
        );
    }
}
