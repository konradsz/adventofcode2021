#[derive(Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32, z: i32) -> Coordinates {
        Coordinates { x, y, z }
    }
}

fn get_orientations(coordinates: Coordinates) -> Vec<Coordinates> {
    let Coordinates { x, y, z } = coordinates;
    vec![
        Coordinates::new(x, y, z),
        Coordinates::new(z, y, -x),
        Coordinates::new(-x, y, -z),
        Coordinates::new(-z, y, x),
        Coordinates::new(-y, x, z),
        Coordinates::new(z, x, y),
        Coordinates::new(y, x, -z),
        Coordinates::new(-z, x, -y),
        Coordinates::new(y, -x, z),
        Coordinates::new(z, -x, -y),
        Coordinates::new(-y, -x, -z),
        Coordinates::new(-z, -x, y),
        Coordinates::new(x, -z, y),
        Coordinates::new(y, -z, -x),
        Coordinates::new(-x, -z, -y),
        Coordinates::new(-y, -z, x),
        Coordinates::new(x, -y, -z),
        Coordinates::new(-z, -y, -x),
        Coordinates::new(-x, -y, z),
        Coordinates::new(z, -y, x),
        Coordinates::new(x, z, -y),
        Coordinates::new(-y, z, -x),
        Coordinates::new(-x, z, y),
        Coordinates::new(y, z, x),
    ]
}

fn main() {}
