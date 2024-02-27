// I kinda doubt we need these tests...
// But I haven't had to implement special array types yet...

// Test if array is created with the specified size
#[test]
fn array_size() {
    let array: [i32; 10] = [0; 10];
    assert_eq!(10, array.len());
}

// Test if array elements can be accessed correctly
#[test]
fn array_access() {
    let mut array: [i32; 10] = [0; 10];
    array[0] = 12;
    array[1] = 157;

    assert_eq!(12, array[0]);
    assert_eq!(157, array[1]);
}
