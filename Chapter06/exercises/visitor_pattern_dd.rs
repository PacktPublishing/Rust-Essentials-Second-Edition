// structs:
struct Point {
    x: f64,
    y: f64
}

struct Circle {
    center: Point,
    radius: f64,
}

struct Square {
    lowerLeftCorner: Point,
    side: f64,
}

// traits:
trait ShapeVisitor {
    fn visit_circle(&mut self, c: &Circle);
    fn visit_square(&mut self, r: &Square);
}

trait Shape {
    fn accept(&self, sv: &mut ShapeVisitor);
}

impl Shape for Circle {
    fn accept(&self, sv: &mut ShapeVisitor) {
        sv.visit_circle(self);
    }
}

impl Shape for Square {
    fn accept(&self, sv: &mut ShapeVisitor) {
        sv.visit_square(self);
    }
}

fn compute_area(s: &Shape) -> f64  {
    struct AreaCalculator {
        area: f64,
    }

    impl ShapeVisitor for AreaCalculator {
        fn visit_circle(&mut self, c: &Circle) {
            self.area = std::f64::consts::PI * c.radius * c.radius;
        }
        fn visit_square(&mut self, r: &Square) {
            self.area = r.side * r.side;
        }
    }
    
    let mut ac = AreaCalculator { area: 0.0 };
    s.accept(&mut ac);
    ac.area
}

fn main() {
    let cn = Point{ x: 0.0, y: 0.0 };
    let ci = Circle{ center: cn, radius: 1.0 };
    let area = compute_area(&ci);
    println!("The area of the circle is {}", area);
    let cn = Point{ x: 0.0, y: 0.0 };
    let sq = Square{ lowerLeftCorner: cn, side: 1.0 };
    let area = compute_area(&sq);
    println!("The area of the square is {}", area);

}
// The area of the circle is 3.141592653589793
// The area of the square is 1