#[derive(Default, Copy, Clone, Debug, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign)]
pub struct Pixel {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}
