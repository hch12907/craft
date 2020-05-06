#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Deg(pub f32);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Rad(pub f32);

impl From<Rad> for Deg {
    fn from(r: Rad) -> Self {
        Self(r.0 * std::f32::consts::FRAC_1_PI * 180.0)
    }
}

impl From<Deg> for Rad {
    fn from(r: Deg) -> Self {
        Self(r.0 * std::f32::consts::PI * 0.00555555555556)
    }
}
