#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn add_points(p: &Point, q: &Point, a: f64) -> Option<Point> {
    if p.x == q.x && p.y == q.y {
        return double_point(p, a);
    }

    if p.x == q.x {
        return None; // Points are inverses, result is the point at infinity.
    }

    // Add a semicolon at the end of this line
    let lambda = (q.y - p.y) / (q.x - p.x);
    let x3 = lambda.powi(2) - p.x - q.x;
    let y3 = lambda * (p.x - x3) - p.y;

    Some(Point { x: x3, y: y3 })
}

fn double_point(p: &Point, a: f64) -> Option<Point> {
    if p.y == 0.0 {
        return None; // Point is at infinity.
    }

    let lambda = (3.0 * p.x.powi(2) + a) / (2.0 * p.y);
    let x3 = lambda.powi(2) - 2.0 * p.x;
    let y3 = lambda * (p.x - x3) - p.y;

    Some(Point { x: x3, y: y3 })
}

fn main() {
    let a = -4.0;
    let _b = 9.0; // Unused variable b for now.

    let p = Point { x: 2.0, y: 3.0 };
    let q = Point { x: 0.0, y: -3.0 };

    match add_points(&p, &q, a) {
        Some(r) => println!("P + Q = {:?}", r),
        None => println!("P + Q is the point at infinity"),
    }

    match double_point(&p, a) {
        Some(r) => println!("2P = {:?}", r),
        None => println!("2P is the point at infinity"),
    }
}

