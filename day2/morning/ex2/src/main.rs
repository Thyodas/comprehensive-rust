use std::f64::consts::PI;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i64,
    y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn dist(&self, other: Point) -> f64 {
        (((other.x - self.x).pow(2)
            + (other.y - self.y).pow(2)) as f64).sqrt()
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    pub fn left_most_point(&self) -> Option<Point> {
        if self.points.is_empty() {
            return None;
        }
        self.points.iter().min_by_key(|point| point.x).cloned()
    }
}

impl Polygon {
    pub fn new() -> Self {
        Self { points: vec![] }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    pub fn perimeter(&self) -> f64 {
        if self.points.is_empty() {
            return 0f64;
        }
        let mut total: f64 = 0f64;
        let mut prev_p: &Point = &self.points[self.points.len() -1];
        for p in &self.points {
            total += p.dist(*prev_p);
            prev_p = p;
        }
        return total;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }
}

pub struct Circle {
    center: Point,
    radius: u64,
}

impl Circle {
    pub fn new(center: Point, radius: u64) -> Circle {
        Circle {
            center,
            radius
        }
    }

    pub fn perimeter(&self) -> f64 {
        2f64 * PI * (self.radius as f64)
    }

    pub fn dist(&self, other: &Self) -> f64 {
        self.center.dist(other.center)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(polygon) => polygon.perimeter(),
            Shape::Circle(circle) => circle.perimeter(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

fn main() {}