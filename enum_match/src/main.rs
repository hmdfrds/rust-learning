use std::f64::consts::PI;

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    fn area(&self) -> Option<f64> {
        match self {
            Shape::Circle { radius } => Some(PI * radius.powi(2)),
            Shape::Rectangle { width, height } => Some(width * height),
            Shape::Triangle { a, b, c } => {
                let a = *a;
                let b = *b;
                let c = *c;

                if a + b <= c || (a + c <= b) || (b + c <= a) || a <= 0.0 || b <= 0.0 || c <= 0.0 {
                    return None;
                }
                let s_semi = (a + b + c) / 2.0;
                let radicand = s_semi * (s_semi - a) * (s_semi - b) * (s_semi - c);
                if radicand < 0.0 {
                    return None;
                }
                Some(radicand.sqrt())
            }
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5f64 },
        Shape::Rectangle {
            width: 5f64,
            height: 4f64,
        },
        Shape::Triangle {
            a: 3f64,
            b: 3f64,
            c: 3f64,
        },
        Shape::Triangle {
            a: 1f64,
            b: 2f64,
            c: 5f64,
        },
    ];

    for shape in shapes.iter() {
        let result = shape.area();
        match result {
            Some(value) => println!("Area is: {}.", value),
            None => println!("Could not calculate area for this shape (e.g., invalid triangle)."),
        }
    }
}
