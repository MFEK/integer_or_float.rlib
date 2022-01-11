use integer_or_float::IntegerOrFloat::{self, *};

#[test]
fn test_impls() {
    assert_eq!(Integer(3) * Float(-1.0), Float(-3.0));
    assert_eq!(Integer(3) * Integer(3), Integer(9));
    assert_eq!(Integer(5) % Integer(2), Integer(1));
    assert_eq!(Float(9.0) - Float(7.0), Float(2.0));
    assert_eq!(-Float(4.0), Float(-4.0));
    assert_eq!(Integer(5) * 5.0, Float(25.0));
    assert_eq!(Float(5.0) * 5, Float(25.0));
    assert_eq!(Integer(5) * 5, Integer(25));
    let x: usize = Integer(5).into();
    assert_eq!(x, 5);
    let x: isize = Integer(-5).into();
    assert_eq!(x, -5);
    let x: f64 = Integer(5).into();
    assert_eq!(x, 5.0);

    let x: IntegerOrFloat = (4.5).into();
    assert_eq!(x, Float(4.5));

    let x: IntegerOrFloat = (10).into();
    assert_eq!(x, Integer(10));
}
