/// A direction such as _Horizontal_ or _Vertical_
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Axis {
    #[default]
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn main<T>(&self, value: impl Into<(T, T)>) -> T {
        let (x, y) = value.into();
        match self {
            Self::Horizontal => x,
            Self::Vertical => y,
        }
    }

    pub fn cross<V>(&self, value: impl Into<(V, V)>) -> V {
        let (x, y) = value.into();
        match self {
            Self::Horizontal => y,
            Self::Vertical => x,
        }
    }

    pub fn pack<T, R>(&self, main: T, cross: T) -> R
    where
        R: From<(T, T)>,
    {
        match self {
            Self::Horizontal => R::from((main, cross)),
            Self::Vertical => R::from((cross, main)),
        }
    }

    pub fn unpack<T>(&self, value: impl Into<(T, T)>) -> (T, T) {
        let (x, y) = value.into();
        match self {
            Self::Horizontal => (x, y),
            Self::Vertical => (y, x),
        }
    }

    pub const fn is_vertical(&self) -> bool {
        matches!(self, Self::Vertical)
    }

    pub const fn is_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal)
    }
}

impl std::ops::Neg for Axis {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}
