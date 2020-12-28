use typsy::anon::Transform;

#[derive(Transform)]
struct Vec3 {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Transform, PartialEq)]
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
fn test() {
    assert_eq!(
        convert(Vec3 {
            w: 0.0,
            x: 1.0,
            y: 2.0,
            z: 3.0
        }),
        Point {
            w: 0.0,
            x: 1.0,
            y: 2.0,
            z: 3.0
        }
    )
}
