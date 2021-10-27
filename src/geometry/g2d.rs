use std::cmp;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

pub struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }
}

/// Represents segnent of line bounded by two points
pub struct Segment<'a> {
    p1: &'a Point,
    p2: &'a Point,
}

/// Represents infinite line
/// A, B, C - coefficients in equation A * X + B * Y = C
#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    A: i32,
    B: i32,
    C: i32,
}

impl Line {
    pub fn new(A: i32, B: i32, C: i32) -> Self {
        if A < 0 || (A == 0 && B < 0) {
            Line {
                A: -A,
                B: -B,
                C: -C,
            }
        } else {
            Line { A, B, C }
        }
    }

    /// todo: what if lines are the same
    pub fn intersection(&self, other: &Line) -> Option<Point> {
        let D = self.A * other.B - self.B * other.A;
        let Dx = self.C * other.B - self.B * other.C;
        let Dy = self.A * other.C - self.C * other.A;
        if D == 0 {
            None
        } else {
            let x = Dx / D;
            let y = Dy / D;
            Some(Point::new(x, y))
        }
    }
}

impl<'a> Segment<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point) -> Self {
        Segment { p1, p2 }
    }

    // taken from here
    // https://stackoverflow.com/questions/20677795/how-do-i-compute-the-intersection-point-of-two-lines
    fn line(&self) -> Line {
        let A = self.p1.y - self.p2.y;
        let B = self.p2.x - self.p1.x;
        let C = -(self.p1.x * self.p2.y - self.p2.x * self.p1.y);
        Line::new(A, B, C)
    }

    // checks whether point p belongs to the box bounded by p1 and p2
    fn is_point_in_box(&self, p: &Point) -> bool {
        let x1 = cmp::min(self.p1.x, self.p2.x);
        let x2 = cmp::max(self.p1.x, self.p2.x);
        let y1 = cmp::min(self.p1.y, self.p2.y);
        let y2 = cmp::max(self.p1.y, self.p2.y);
        x1 <= p.x && p.x <= x2 && y1 <= p.y && p.y <= y2
    }

    /// Returns intersection point of two segnemts (if there is any)
    /// todo: edge cases
    ///     * two segments on the same line
    pub fn intersection(&self, other: &Segment) -> Option<Point> {
        let l1 = self.line();
        let l2 = other.line();
        if self.line() == other.line() {
            if self.is_point_in_box(&other.p1) {
                Some(*other.p1)
            } else if self.is_point_in_box(&other.p2) {
                Some(*other.p2)
            } else if other.is_point_in_box(&self.p1) {
                Some(*self.p1)
            } else if other.is_point_in_box(&self.p2) {
                Some(*self.p2)
            } else {
                None
            }
        } else {
            self.line()
                .intersection(&other.line())
                .filter(|p| self.is_point_in_box(p) && other.is_point_in_box(p))
        }
    }
}

struct Triangle<'a> {
    p1: & 'a Point,
    p2: & 'a Point,
    p3: & 'a Point,
}

impl <'a>Triangle<'a> {
    pub fn new(p1: & 'a Point, p2: & 'a Point, p3: & 'a Point) -> Self {
        Triangle { p1, p2, p3 }
    }

    fn sign(p1: &Point, p2: &Point, p3: &Point) -> i8 {
        let ind = (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y);
        if ind > 0 {
            1
        } else if ind < 0 {
            -1
        } else {
            0
        }
    }

    /// checks whether point is inside tiangle
    /// https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
    fn is_inside(&self, p: &Point) -> bool {
        let d1 = Self::sign(p, &self.p1, &self.p2);
        let d2 = Self::sign(p, &self.p2, &self.p3);
        let d3 = Self::sign(p, &self.p3, &self.p1);

        let has_neg = d1 < 0 || d2 < 0 || d3 < 0;
        let has_pos = d1 > 0 || d2 > 0 || d3 > 0;
        !(has_neg && has_pos)
    }

    /// returns any intersection point
    pub fn intersection(&self, other: &Triangle) -> Option<Point> {
        // if one triangle inside other
        if self.is_inside(&other.p1) {
            return Some(*other.p1);
        } else if other.is_inside(&self.p1) {
            return Some(*self.p1);
        }

        // checking that any segments are intersecting
        for (p1, p2) in [(self.p1, self.p2), (self.p2, self.p3), (self.p3, self.p1)].iter() {
            for (q1, q2) in [
                (other.p1, other.p2),
                (other.p2, other.p3),
                (other.p3, other.p1),
            ]
            .iter()
            {
                let seg1 = Segment::new(p1, p2);
                let seg2 = Segment::new(q1, q2);
                let intersection = seg1.intersection(&seg2);
                if intersection.is_some() {
                    return intersection;
                }

            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let x = Point::new(1, 3);
    }

    #[test]
    fn segments_intersection() {
        let (p1, p2) = (Point::new(-1, 0), &Point::new(2, 0));
        let seg1 = Segment::new(&p1, &p2);
        let (p1, p2) = (Point::new(0, -1), Point::new(0, 2));
        let seg2 = Segment::new(&p1, &p2);
        assert_eq!(seg1.intersection(&seg2), Some(Point::new(0, 0)));

        let (p2, p1) = (Point::new(1, 1), Point::new(-1, -1));
        let seg1 = Segment::new(&p1, &p2);
        let (p1, p2) = (Point::new(1, -1), Point::new(-2, 2));
        let seg2 = Segment::new(&p1, &p2);
        assert_eq!(seg1.intersection(&seg2), Some(Point::new(0, 0)));

        let (p1, p2) = (Point::new(1, 1), Point::new(-1, 1));
        let seg1 = Segment::new(&p1, &p2);
        let (p1, p2) = (Point::new(1, -1), Point::new(-1, -2));
        let seg2 = Segment::new(&p1, &p2);
        assert_eq!(seg1.intersection(&seg2), None);

        let (p1, p2) = (Point::new(1, 0), Point::new(0, 0));
        let seg1 = Segment::new(&p1, &p2);
        let (p1, p2) = (Point::new(-1, 0), Point::new(0, 0));
        let seg2 = Segment::new(&p1, &p2);
        assert_eq!(seg1.intersection(&seg2), Some(Point::new(0, 0)));

        let (p1, p2) = (Point::new(2, 0), Point::new(0, 0));
        let seg1 = Segment::new(&p1, &p2);
        let (p1, p2) = (Point::new(-1, 0), Point::new(1, 0));
        let seg2 = Segment::new(&p1, &p2);
        assert_eq!(seg1.intersection(&seg2), Some(Point::new(1, 0)));
    }

    #[test]
    fn is_inside() {
        let (p1, p2, p3) = (Point::new(0, 0), Point::new(2, 2), Point::new(-2, 2));

        let triangle = Triangle::new(&p1, &p2, &p3);
        assert!(triangle.is_inside(&Point::new(0, 0)));
        assert!(triangle.is_inside(&Point::new(0, 1)));
        assert!(!triangle.is_inside(&Point::new(-1, -1)));
    }

    #[test]
    fn triangls_intersection_one_point() {
        let (p1, p2, p3) = (Point::new(0, 0), Point::new(2, 2), Point::new(-2, 2));
        let (q1, q2, q3) = (Point::new(0, 0), Point::new(2, -2), Point::new(-2, -2));

        let triangle1 = Triangle::new(&p1, &p2, &p3);
        let triangle2 = Triangle::new(&q1, &q2, &q3);

        assert_eq!(triangle1.intersection(&triangle2), Some(Point::new(0,0)));
    }

    #[test]
    fn triangls_intersection_inside() {
        let (p1, p2, p3) = (Point::new(0, 0), Point::new(10, 5), Point::new(-10, 5));
        let (q1, q2, q3) = (Point::new(0, 1), Point::new(1, 2), Point::new(-1, 2));

        let triangle1 = Triangle::new(&p1, &p2, &p3);
        let triangle2 = Triangle::new(&q1, &q2, &q3);

        assert_eq!(triangle1.intersection(&triangle2), Some(Point::new(0,1)));
    }

    #[test]
    fn triangls_intersection_dont_intersect() {
        let (p1, p2, p3) = (Point::new(1, 1), Point::new(2, 2), Point::new(-2, 2));
        let (q1, q2, q3) = (Point::new(0, 0), Point::new(2, -2), Point::new(-2, -2));

        let triangle1 = Triangle::new(&p1, &p2, &p3);
        let triangle2 = Triangle::new(&q1, &q2, &q3);

        assert_eq!(triangle1.intersection(&triangle2), None);
    }
}
