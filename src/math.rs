//! Common helpers and utils

pub trait SubAbs {
    /// The absolute value of the difference between two values
    fn sub_abs(self, other: Self) -> Self;
}

impl SubAbs for u8 {
    fn sub_abs(self, other: Self) -> Self {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl SubAbs for u16 {
    fn sub_abs(self, other: Self) -> Self {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl SubAbs for u32 {
    fn sub_abs(self, other: Self) -> Self {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl SubAbs for u128 {
    fn sub_abs(self, other: Self) -> Self {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl SubAbs for usize {
    fn sub_abs(self, other: Self) -> Self {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl SubAbs for i8 {
    fn sub_abs(self, other: Self) -> Self {
        (self - other).abs()
    }
}

impl SubAbs for i16 {
    fn sub_abs(self, other: Self) -> Self {
        (self - other).abs()
    }
}

impl SubAbs for i32 {
    fn sub_abs(self, other: Self) -> Self {
        (self - other).abs()
    }
}

impl SubAbs for i128 {
    fn sub_abs(self, other: Self) -> Self {
        (self - other).abs()
    }
}

impl SubAbs for isize {
    fn sub_abs(self, other: Self) -> Self {
        (self - other).abs()
    }
}

impl SubAbs for f32 {
    fn sub_abs(self, other: Self) -> Self {
        (self - other).abs()
    }
}

impl SubAbs for f64 {
    fn sub_abs(self, other: Self) -> Self {
        (self - other).abs()
    }
}