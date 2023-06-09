use super::clr::Clr;
use approx::assert_relative_eq;

#[test]
fn add_colors() {
    let c1 = Clr::rgb(0.9, 0.6, 0.75);
    let c2 = Clr::rgb(0.7, 0.1, 0.25);
    let c3 = c1 + c2;
    assert_eq!(c3.r(), 1.0);
    assert_relative_eq!(c3.g(), 0.7, epsilon = 1e-5);
    assert_eq!(c3.b(), 1.0);
}

#[test]
fn sub_colors() {
    let c1 = Clr::rgb(0.9, 0.6, 0.75);
    let c2 = Clr::rgb(0.7, 0.8, 0.25);
    let c3 = c1 - c2;
    assert_relative_eq!(c3.r(), 0.2, epsilon = 1e-5);
    assert_eq!(c3.g(), 0.0);
    assert_eq!(c3.b(), 0.5);
}

#[test]
fn scalar_multiply() {
    let c1 = Clr::rgb(0.2, 0.3, 0.4);
    assert_eq!(c1 * 2.0, Clr::rgb(0.4, 0.6, 0.8));
}

#[test]
fn color_multiply() {
    let c1 = Clr::rgb(1.0, 0.2, 0.4);
    let c2 = Clr::rgb(0.9, 1.0, 0.1);
    let c3 = c1 * c2;
    assert_eq!(c3.r(), 0.9);
    assert_eq!(c3.g(), 0.2);
    assert_relative_eq!(c3.b(), 0.04, epsilon = 1e-5);
}
