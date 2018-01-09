//! Simple macros to declare 'type and range checked' aliases for integers and floats.
//!
//! # Examples
//! Basic usage: just aliasing float.
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
//!
//! In addition, you can declare an alias with a 'range checked' constructor.
//!
//! ```should_panic
//! #[macro_use]
//! extern crate num_alias;
//! fn main() {
//!     // declare alias with range[3, 6)
//!     int_alias!(Val, i32, 3 => 6);
//!     let a = Val(5);
//!     let b = Val(4);
//!     // this code panics
//!     let c = a * b;
//! }
//! ```
//! **Note:** When using with range, these macros declare aliases *not* as Tuple Struct, but as Normal Struct and declare global functions with their name(In above example, ```fn Val(i: i32)-> Val``` is declared).
//!
//! This spec is for ease of use, but make alias' range safety imcomplete(If you can construct an alias with default constructor(like ```Val{inner: 5}```), range cheking won't run.)
//!

/// A simple macro to declare alias for Integer types and implement arithmetics.
///
/// Concretely, it implements for the alias type, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Deref(for accessing inner value).
///
/// In addition, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd are Derived.
///
/// If not declared with range, Default is also derived.
///
/// # Examples
/// Basic usage: just aliasing int.
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
///
/// In addition, you can declare an alias with a 'range checked' constructor.
///
/// ```should_panic
/// #[macro_use]
/// extern crate num_alias;
/// fn main() {
///     // declare alias with range[3, 6)
///     int_alias!(Val, i32, 3 => 6);
///     let a = Val(5);
///     let b = Val(4);
///     // this code panics
///     let c = a * b;
/// }
/// ```
#[macro_export]
macro_rules! int_alias {
    ($alias:ident, $type:ty) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $alias(pub $type);
        impl ::std::ops::Deref for $alias {
            type Target = $type;
            fn deref(&self) -> &$type {
                &self.0
            }
        }
        __impl_arithmetic!($alias, $type);
    };
    ($alias:ident, $type:ty, $lb:expr => $hb:expr) => {
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $alias {
            inner: $type
        }
        #[allow(non_snake_case)]
        pub fn $alias(i: $type) -> $alias {
            assert!($lb as $type <= i  && i < $hb as $type,
                    concat!("Invalid value for ", stringify!($alias), ": {}"), i);
            $alias {
                inner: i
            }
        }
        impl ::std::ops::Deref for $alias {
            type Target = $type;
            fn deref(&self) -> &$type {
                &self.inner
            }
        }
        __impl_arithmetic!($alias, $type);
    }
}

/// A simple macro to declare alias for Integer types and implement arithmetics.
///
/// Concretely, it implements for the alias type, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Deref(for accessing inner value).
///
/// In addition, Clone, Copy, Debug, Default, PartialEq, PartialOrd are Derived.
///
/// If not declared with range, Default is also derived.
///
/// # Examples
/// Basic usage: just aliasing float.
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
///
/// In addition, you can declare an alias with a 'range checked' constructor.
///
/// ```should_panic
/// #[macro_use]
/// extern crate num_alias;
/// fn main() {
///     // declare alias with range[3.0, 6.0)
///     float_alias!(Fval, f64, 3 => 6);
///     let a = Fval(5.0);
///     let b = Fval(4.0);
///     // this code panics
///     let c = (a * -b).sqrt();
/// }
/// ```
#[macro_export]
macro_rules! float_alias {
    ($alias:ident, $type:ty) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
        pub struct $alias(pub $type);
        impl ::std::ops::Deref for $alias {
            type Target = $type;
            fn deref(&self) -> &$type {
                &self.0
            }
        }
        __impl_arithmetic!($alias, $type);
        __impl_neg!($alias);
    };
    ($alias:ident, $type:ty, $lb:expr => $hb:expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $alias {
            inner: $type
        }
        #[allow(non_snake_case)]
        pub fn $alias(i: $type) -> $alias {
            assert!($lb as $type <= i  && i < $hb as $type,
                    concat!("Invalid value for ", stringify!($alias), ": {}"), i);
            $alias {
                inner: i
            }
        }
        impl ::std::ops::Deref for $alias {
            type Target = $type;
            fn deref(&self) -> &$type {
                &self.inner
            }
        }
        __impl_arithmetic!($alias, $type);
        __impl_neg!($alias);
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_neg {
    ($alias:ident) => {
        impl ::std::ops::Neg for $alias {
            type Output = $alias;
            fn neg(self) -> $alias {
                $alias(-*self)
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_arith_consume {
    ($alias:ident, $type:ty, $tr:path, $func:ident) => {
        impl $tr for $alias {
            type Output = $alias;
            fn $func(self, other: $alias) -> $alias {
                $alias((*self).$func(*other))
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_assign_consume {
    ($alias:ident, $type:ty, $tr:path, $func:ident, $ope:tt) => {
        impl $tr for $alias {
            fn $func(&mut self, other: $alias) {
                *self = *self $ope other;
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_arithmetic {
    ($alias:ident, $type:ty) => {
        __impl_arith_consume!($alias, $type, ::std::ops::Add, add);
        __impl_arith_consume!($alias, $type, ::std::ops::Sub, sub);
        __impl_arith_consume!($alias, $type, ::std::ops::Mul, mul);
        __impl_arith_consume!($alias, $type, ::std::ops::Div, div);
        __impl_arith_consume!($alias, $type, ::std::ops::Rem, rem);
        __impl_assign_consume!($alias, $type, ::std::ops::AddAssign, add_assign, +);
        __impl_assign_consume!($alias, $type, ::std::ops::SubAssign, sub_assign, -);
        __impl_assign_consume!($alias, $type, ::std::ops::MulAssign, mul_assign, *);
        __impl_assign_consume!($alias, $type, ::std::ops::DivAssign, div_assign, /);
        __impl_assign_consume!($alias, $type, ::std::ops::RemAssign, rem_assign, %);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn normal_i32() {
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
    }

    #[test]
    fn normal_u32() {
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
    }

    #[test]
    fn normal_f64() {
        float_alias!(Fval, f64);
        let a = Fval(20.0);
        let b = Fval(6.0);
        assert_eq!(a + b, Fval(26.0));
        assert_eq!(a - b, Fval(14.0));
        assert_eq!(a * b, Fval(120.0));
        assert_eq!(a / b, Fval(20.0 / 6.0));
        assert_eq!(a % b, Fval(2.0));
        assert_eq!(-a, Fval(-20.0));
        assert_eq!(a.sqrt(), 20.0f64.sqrt());
        assert_eq!(0.0, *Fval::default());
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn panick_if_out_of_range_int() {
        int_alias!(Val, i32, 3 => 6);
        let a = Val(3);
        let b = Val(4);
        let c = a + b;
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn panick_if_out_of_range_float() {
        float_alias!(Fval, f64, 3 => 6);
        let a = Fval(3.4);
        let b = Fval(4.2);
        let c = -b + a;
    }
}
