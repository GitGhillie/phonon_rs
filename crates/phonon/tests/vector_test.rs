use glam::{Vec2, Vec3, Vec4};
use std::mem::size_of;

#[test]
fn vector_size() {
    assert_eq!(8, size_of::<Vec2>());
    assert_eq!(12, size_of::<Vec3>());
    assert_eq!(16, size_of::<Vec4>());
}

// todo: Check if glam already has all the test cases for Vector
