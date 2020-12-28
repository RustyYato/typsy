use typsy::{anon, anon::Transform, Anon};

#[derive(Transform)]
struct Vec3 {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: Extra,
}

#[derive(Transform)]
struct Extra {
    pub value: f32,
}

#[derive(Transform, Debug, PartialEq)]
struct Point {
    pub y: f32,
    pub w: f32,
    pub z: Anon!(value: f32),
}

#[derive(Transform)]
struct TuplePoint(pub f32, pub i32, pub u32);

fn convert(vec: Vec3) -> Point { vec.deep_transform() }

#[test]
fn test() {
    assert_eq!(
        convert(Vec3 {
            w: 0.0,
            x: 1.0,
            y: 2.0,
            z: Extra { value: 3.0 },
        }),
        Point {
            w: 0.0,
            z: anon!(value = 3.0),
            y: 2.0,
        }
    )
}
