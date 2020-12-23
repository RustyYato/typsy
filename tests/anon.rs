use typsy::anon::Transform;

#[derive(Transform)]
struct Vec3 {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Transform)]
struct Point {
    pub y: f32,
    pub w: f32,
    pub z: f32,
    pub x: f32,
}

#[derive(Transform)]
struct TuplePoint(pub f32, pub i32, pub u32);

fn convert(vec: Vec3) -> Point { vec.transform() }

#[test]
fn test() {}
