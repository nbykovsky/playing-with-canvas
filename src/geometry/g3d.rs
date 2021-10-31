use super::g2d;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    pub fn project(&self) -> g2d::Point {
        g2d::Point::new(self.x, self.y)
    }

    fn sub(&self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// https://www.khanacademy.org/math/multivariable-calculus/thinking-about-multivariable-function/x786f2022:vectors-and-matrices/a/cross-products-mvc
    fn x_product(&self, other: &Point) -> Point {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Point::new(x, y, z)
    }

    fn dot_product(&self, other: &Point) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Triagnle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triagnle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triagnle { p1, p2, p3 }
    }

    /// if projection is a line it will return None
    pub fn project(&self) -> Option<g2d::Triangle> {
        let p1 = self.p1.project();
        let p2 = self.p2.project();
        let p3 = self.p3.project();

        let on_the_same_line = (p1.y - p2.y) * (p1.x - p3.x) == (p1.y - p3.y) * (p1.x - p2.x);
        if on_the_same_line {
            None
        } else {
            Some(g2d::Triangle::new(p1, p2, p3))
        }
    }

    /// if true other should be painted BEFORE self
    pub fn is_above(&self, other: &Triagnle) -> bool {
        match (self.project(), other.project()) {
            (Some(slf), Some(otr)) => {
                if let Some(point) = slf.intersection(&otr) {
                    let self_h = Plane::from_triangle(&self).get_z(point.x, point.y).unwrap();
                    let other_h = Plane::from_triangle(&other)
                        .get_z(point.x, point.y)
                        .unwrap();
                    self_h > other_h
                } else {
                    false
                }
            }
            _ => false,
        }
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
    fn from_triangle(triangle: &Triagnle) -> Self {
        let v1 = triangle.p3.sub(&triangle.p1);
        let v2 = triangle.p2.sub(&triangle.p1);
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
        let triangle = Triagnle::new(
            Point::new(0, 0, 0),
            Point::new(-1, 1, 0),
            Point::new(1, 1, 0),
        );
        let plane = Plane::from_triangle(&triangle);

        assert_eq!(plane.get_z(0, 0), Some(0));
    }

    #[test]
    fn parallel_triangles() {
        let triangle1 = Triagnle::new(
            Point::new(0, 0, 0),
            Point::new(-1, 1, 0),
            Point::new(1, 1, 0),
        );
        let triangle2 = Triagnle::new(
            Point::new(0, 0, 1),
            Point::new(-1, 1, 1),
            Point::new(1, 1, 1),
        );

        assert!(!triangle1.is_above(&triangle2));
        assert!(triangle2.is_above(&triangle1))
    }

    #[test]
    fn overlapping_triangles() {
        let triangle1 = Triagnle::new(
            Point::new(0, 0, 0),
            Point::new(-1, 1, -1),
            Point::new(1, 1, -1),
        );
        let triangle2 = Triagnle::new(
            Point::new(0, 0, 1),
            Point::new(-1, -1, 2),
            Point::new(1, -1, 2),
        );

        assert!(!triangle1.is_above(&triangle2));
        assert!(triangle2.is_above(&triangle1));
    }

    #[test]
    fn non_overlapping_triangles() {
        let triangle1 = Triagnle::new(
            Point::new(0, 1, 0),
            Point::new(-1, 2, -1),
            Point::new(1, 2, -1),
        );
        let triangle2 = Triagnle::new(
            Point::new(0, 0, 1),
            Point::new(-1, -1, 2),
            Point::new(1, -1, 2),
        );

        assert!(!triangle1.is_above(&triangle2));
        assert!(!triangle2.is_above(&triangle1));
    }
}
