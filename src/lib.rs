//! Simple macros to declare 'type checked' aliases for integers and floats.
//!
//! # Examples
//!
//! ```
//! #[macro_use]
//! extern crate num_alias;
//! fn main() {
//!     float_alias!(Fval, f64);
//!     let a = Fval(5.0);
//!     let b = Fval(4.0);
//!     let c = (a * b).sqrt();
//! }
//! ```

/// A simple macro to declare alias for Integer types and implement arithmetics.
///
/// Concretely, it implements for the alias type, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Deref(for accessing inner value).
///
/// In addition, Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOr are Derived.
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate num_alias;
/// fn main() {
///     int_alias!(Val, i32);
///     let a = Val(5);
///     let b = Val(4);
///     let c = a * b;
/// }
/// ```
#[macro_export]
macro_rules! int_alias {
    ($alias:ident, $type:ty) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $alias($type);
        impl ::std::ops::Deref for $alias {
            type Target = $type;
            fn deref(&self) -> &$type {
                &self.0
            }
        }
        impl ::std::ops::Add for $alias {
            type Output = $alias;
            fn add(self, other: $alias) -> $alias {
                $alias(self.0 + other.0)
            }
        }
        impl ::std::ops::Sub for $alias {
            type Output = $alias;
            fn sub(self, other: $alias) -> $alias {
                $alias(self.0 - other.0)
            }
        }
        impl ::std::ops::Mul for $alias {
            type Output = $alias;
            fn mul(self, other: $alias) -> $alias {
                $alias(self.0 * other.0)
            }
        }
        impl ::std::ops::Div for $alias {
            type Output = $alias;
            fn div(self, other: $alias) -> $alias {
                $alias(self.0 / other.0)
            }
        }
        impl ::std::ops::Rem for $alias {
            type Output = $alias;
            fn rem(self, other: $alias) -> $alias {
                $alias(self.0 % other.0)
            }
        }
        impl ::std::ops::AddAssign for $alias {
            fn add_assign(&mut self, other: $alias) {
                *self = *self + other
            }
        }
        impl ::std::ops::SubAssign for $alias {
            fn sub_assign(&mut self, other: $alias) {
                *self = *self - other
            }
        }
        impl ::std::ops::MulAssign for $alias {
            fn mul_assign(&mut self, other: $alias) {
                *self = *self * other
            }
        }
        impl ::std::ops::DivAssign for $alias {
            fn div_assign(&mut self, other: $alias) {
                *self = *self / other
            }
        }
        impl ::std::ops::RemAssign for $alias {
            fn rem_assign(&mut self, other: $alias) {
                *self = *self % other
            }
        }
    }
}

/// A simple macro to declare alias for Integer types and implement arithmetics.
///
/// Concretely, it implements for the alias type, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Deref(for accessing inner value).
///
/// In addition, Clone, Copy, Debug, Default, PartialEq, PartialOrd are Derived.
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate num_alias;
/// fn main() {
///     float_alias!(Fval, f64);
///     let a = Fval(5.0);
///     let b = Fval(4.0);
///     let c = (a * b).sqrt();
/// }
/// ```
#[macro_export]
macro_rules! float_alias {
    ($alias:ident, $type:ty) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
        pub struct $alias($type);
        impl ::std::ops::Deref for $alias {
            type Target = $type;
            fn deref(&self) -> &$type {
                &self.0
            }
        }
        impl ::std::ops::Add for $alias {
            type Output = $alias;
            fn add(self, other: $alias) -> $alias {
                $alias(self.0 + other.0)
            }
        }
        impl ::std::ops::Sub for $alias {
            type Output = $alias;
            fn sub(self, other: $alias) -> $alias {
                $alias(self.0 - other.0)
            }
        }
        impl ::std::ops::Mul for $alias {
            type Output = $alias;
            fn mul(self, other: $alias) -> $alias {
                $alias(self.0 * other.0)
            }
        }
        impl ::std::ops::Div for $alias {
            type Output = $alias;
            fn div(self, other: $alias) -> $alias {
                $alias(self.0 / other.0)
            }
        }
        impl ::std::ops::Rem for $alias {
            type Output = $alias;
            fn rem(self, other: $alias) -> $alias {
                $alias(self.0 % other.0)
            }
        }
        impl ::std::ops::AddAssign for $alias {
            fn add_assign(&mut self, other: $alias) {
                *self = *self + other
            }
        }
        impl ::std::ops::SubAssign for $alias {
            fn sub_assign(&mut self, other: $alias) {
                *self = *self - other
            }
        }
        impl ::std::ops::MulAssign for $alias {
            fn mul_assign(&mut self, other: $alias) {
                *self = *self * other
            }
        }
        impl ::std::ops::DivAssign for $alias {
            fn div_assign(&mut self, other: $alias) {
                *self = *self / other
            }
        }
        impl ::std::ops::RemAssign for $alias {
            fn rem_assign(&mut self, other: $alias) {
                *self = *self % other
            }
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        int_alias!(Val, i32);
        let a = Val(20);
        assert_eq!(*a, 20);
        let b = Val(6);
        assert_eq!(a + b, Val(26));
        assert_eq!(a - b, Val(14));
        assert_eq!(a * b, Val(120));
        assert_eq!(a / b, Val(3));
        assert_eq!(a % b, Val(2));
        assert_eq!(0, *Val::default());
        float_alias!(Fval, f64);
        let a = Fval(20.0);
        let b = Fval(6.0);
        assert_eq!(a + b, Fval(26.0));
        assert_eq!(a - b, Fval(14.0));
        assert_eq!(a * b, Fval(120.0));
        assert_eq!(a / b, Fval(20.0 / 6.0));
        assert_eq!(a % b, Fval(2.0));
        assert_eq!(a.sqrt(), 20.0f64.sqrt());
        assert_eq!(0.0, *Fval::default());
    }
}
