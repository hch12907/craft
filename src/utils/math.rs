// --- Mathematical utilities start here ---

use std::ops::{ Add, Sub, Mul };

pub fn lerp<T>(x: T, y: T, t: T) -> T 
    where T: Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Copy
{
    x + t * (y - x)
}

pub fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0  - 15.0) + 10.0)
}
