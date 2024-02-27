use biquad::*;

#[test]
fn iir_filter() {
    let coefficients = Coefficients::<f32> {
        a1: 2.0,
        a2: 3.0,
        b0: 4.0,
        b1: 5.0,
        b2: 6.0,
    };

    let mut biquad1 = DirectForm1::<f32>::new(coefficients);

    let dry = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mut wet: [f32; 5] = [0.0; 5];

    for i in 0..dry.len() {
        wet[i] = biquad1.run(dry[i]);
    }

    assert_eq!([4.0, 5.0, 6.0, 16.0, 8.0], wet);
}
