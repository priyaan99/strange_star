#[derive(Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All NAN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0);

    /// A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0);

    /// A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0);

    /// A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x, y,
        }
    } 

    pub fn add_val(&self, val: f32) -> Self {
        Self { x: self.x+val, y: self.y+val }
    }

    pub fn mul_val(&self, val: f32) -> Self {
        Self { x: self.x*val, y: self.y*val }
    }

    pub fn add(&self, x: Self) -> Self {
        Self {
            x: self.x + x.x,
            y: self.y + x.y,
        }
    }

    pub fn lerp(&self, dest: Self, amount: f32) -> Self {
        Self {
            x: self.x + amount * (dest.x - self.x),
            y: self.y + amount * (dest.y - self.y),
        }
    }

    pub fn clamp(&self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}
