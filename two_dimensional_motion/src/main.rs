use std::fmt;
use std::ops;

// model a point in 2D space
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new() -> Self {
        Point { x: 0f64, y: 0f64 }
    }

    fn quadrant(&self) -> u32 {
        let Point { x, y } = self;
        let x_pos = *x >= 0f64;
        let y_pos = *y >= 0f64;

        if x_pos && y_pos {
            1
        } else if !x_pos && y_pos {
            2
        } else if !x_pos && !y_pos {
            3
        } else {
            4
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// // overloaded add operator
// impl ops::Add<Force> for Point {
//     type Output = Point;

//     fn add(self, _rhs: Force) -> Point {
//         self
//     }
// }

// model a force with an angle and a magnitude
// the angle, in degrees, start from the line out in the positive x direction of the cartesian plane, and wraps around 360 degrees
struct Force {
    angle: f64,
    magnitude: u32,
}

impl Force {
    fn new() -> Self {
        Force {
            angle: 0f64,
            magnitude: 0,
        }
    }
}

// overloaded multiply operator
// impl ops::Mul<f64> for Force {}

impl fmt::Display for Force {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} degrees with magnitude of {}",
            self.angle, self.magnitude
        )
    }
}

fn main() {
    println!("hello world");
    // create runner loop that reads input args
    // and print them out

    // todo
    // - overloaded addition operator between Point object and Force object
    // - overloaded multiplication operator between a Force object, and a float scalar

    // not doing
    // - constructors with default args (not supported)
    // - no overloaded input operator

    let p = Point::new();
    println!("{p}")
}
