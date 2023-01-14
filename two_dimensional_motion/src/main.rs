use std::f64::consts::PI;
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
impl ops::Add<Force> for Point {
    type Output = Self;

    fn add(self, f: Force) -> Point {
        let mag = f.magnitude;
        let rads = f.angle * (PI / 360f64);

        let dx = mag * rads.cos();
        let dy = mag * rads.sin();
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

// model a force with an angle and a magnitude
// the angle, in degrees, start from the line out in the positive x direction of the cartesian plane, and wraps around 360 degrees
struct Force {
    angle: f64,
    magnitude: f64,
}

impl Force {
    fn new() -> Self {
        Force {
            angle: 0f64,
            magnitude: 0f64,
        }
    }
}

// overloaded multiply operator
impl ops::Mul<f64> for Force {
    type Output = Self;

    fn mul(self, scalar: f64) -> Force {
        Force {
            angle: self.angle,
            magnitude: self.magnitude * scalar,
        }
    }
}

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
    let mut line = String::new();
    let mut force = Force::new();
    let mut point = Point::new();

    // todo: how to read input stream one char at a time
    loop {
        let res = std::io::stdin().read_line(&mut line);
        match res {
            Err(_) => break,
            _ => (),
        }

        match line.as_str() {
            "n" => {}
            "p" => {}
            "+" => {}
            "*" => {}
            "q" => {}
            _ => {
                println!("Invalid test case!");
                println!("Invalid command: '{line}'");
                break;
            }
        }
    }

    println!("exiting!")
}
