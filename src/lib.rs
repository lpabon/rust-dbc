// Copyright 2017 Luis Pabón <lpabon@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! # Design-by-Contract for Rust
//!
//! Design by Contract is a programming methodology
//! which binds the caller and the function called to a
//! contract. The contract is represented using Hoare Triple:
//! `{P} C {Q}`, where `{P}` is the precondition before
//! executing command `C`, and `{Q}` is the postcondition.
//!
//! Like `debug_assert!`, dbc macros are only enabled in non
//! optimized builds by default. An optimized build will omit all
//! dbc macro statements unless `-C debug-assertions` is passed to the
//! compiler.
//!
//! ## See Also
//!
//! * http://en.wikipedia.org/wiki/Design_by_contract
//! * http://en.wikipedia.org/wiki/Hoare_logic
//! * https://dlang.org/spec/contracts.html
//!
//! ## Note
//! This library is similar to my [godbc](https://github.com/lpabon/godbc)
//! for Golang.
//!

#[doc(hidden)]
#[macro_export]
macro_rules! dbc_panic {
    ($type:expr, $cond:expr) => (if !$cond {
        use std::env;
        println!("panic: {}: \nfile: {}:{}",
            $type, file!(), line!());
        env::set_var("RUST_BACKTRACE", "1");
        assert!($cond);
    });
    ($type:expr, $cond:expr, $($args:tt)*) => (if !$cond {
        use std::env;
        println!("panic: {}: \nfile: {}:{}",
            $type, file!(), line!());
        println!("vars:\n{}", formatvar!($($args)*));
        env::set_var("RUST_BACKTRACE", "1");
        assert!($cond);
    })
}

/// Stringify one or more variables and their values
///
/// This macro is used by other macros in `rust-dbc` to output
/// the variables requested and their values. Structs can be
/// displayed only if they have the `[#derive(Debug)]` attribute.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate dbc;
///
/// # fn main() {
/// #[derive(Debug)]
/// struct AA(i32);
///
/// #[derive(Debug)]
/// struct BB(AA);
///
/// let a = 34;
/// let b = BB(AA(234));
/// let msg = "My message";
///
/// // Output: a=34
/// println!("{}", formatvar!(a));
///
/// // Output: b=BB(AA(234))
/// println!("{}", formatvar!(b));
///
/// // Output: msg="My message" a=34 b=BB(AA(234))
/// println!("{}", formatvar!(msg,a,b));
/// # }
/// ```
#[macro_export]
macro_rules! formatvar {
    ($var:ident) => (format!("{}={:?}", stringify!($var), $var));
    ($var:ident, $($arg:tt)*) => (format!("{} {}", formatvar!($var), formatvar!($($arg)*)));
}

/// Precondondition tests
///
/// This macro is used to assert preconditions. Any variables passed
/// will stringify both the variable and their values. Structs can be
/// displayed only if they have the `[#derive(Debug)]` attribute.
///
/// # Examples
///
/// ```should_panic
/// # #![allow(unreachable_code)]
/// # #[macro_use] extern crate dbc;
///
/// fn foo(x: i32, y: i32) {
///     require!(x != 0, x, y);
///     require!(y < 0, x, y);
///
///     // Do some work here ...
/// }
///
/// fn main() {
///     // No asserts
///     foo(10, -100);
///
///     // Require fails
///     foo(0, 100);
///
///     // Require fails
///     foo(10, 100);
/// }
/// ```
#[macro_export]
macro_rules! require {
    ($cond:expr) => (if cfg!(debug_assertions) {
        dbc_panic!("REQUIRE", $cond)
    });
    ($cond:expr, $($args:tt)*) => (if cfg!(debug_assertions) {
        dbc_panic!("REQUIRE", $cond, $($args)*)
    })
}

/// Postcondition tests
///
/// This macro is used to assert postconditions. Any variables passed
/// will stringify both the variable and their values. Structs can be
/// displayed only if they have the `[#derive(Debug)]` attribute.
///
/// # Examples
///
/// ```should_panic
/// # #![allow(unreachable_code)]
/// # #[macro_use] extern crate dbc;
///
/// fn foo(x: i32, y: i32) -> i32 {
///     require!(x != 0, x, y);
///     require!(y < 0, x, y);
///
///     let sum = x + y;
///
///     // Ensure passes
///     ensure!(sum == x + y, sum, x, y);
///
///     // Ensure fails
///     ensure!(sum != x + y, sum, x, y);
///
///     sum
/// }
///
/// fn main() {
///     // No asserts
///     let a = foo(10, -100);
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr) => (if cfg!(debug_assertions) {
        dbc_panic!("ENSURE", $cond)
    });
    ($cond:expr, $($args:tt)*) => (if cfg!(debug_assertions) {
        dbc_panic!("ENSURE", $cond, $($args)*)
    })
}

/// The `Invariant` trait allows for asserting an object
///
/// Implementors of the `Invariant` trait can then use the `invariant!`
/// macro to test their objects are safe to use.
///
/// See the `invariant!` macro for examples
pub trait Invariant {
    fn invariant(&self) -> bool;
}

/// Invariant condition assertion
///
/// Checks that an object condition is true at all times. The
/// object must implement the `Invariant` trait.
///
/// # Examples
///
/// ```should_panic
/// # #![allow(unreachable_code)]
/// # #[macro_use] extern crate dbc;
/// use dbc::Invariant;
/// #[derive(Debug)]
/// struct Rectangle {
///     length: i32,
///     width: i32,
/// }
///
/// impl Rectangle {
///     fn area(&self) -> i32 {
///         invariant!(self);
///
///         self.length * self.width
///     }
/// }
///
/// impl Invariant for Rectangle {
///     fn invariant(&self) -> bool {
///         self.length > 0 && self.width > 0
///     }
/// }
///
/// fn main() {
///     let msg = "My message";
///     let r = Rectangle{
///        length: 100,
///        width: 0,
///     };
///
///     println!("Area is {:?}", r.area());
/// }
/// ```
#[macro_export]
macro_rules! invariant {
    ($obj:ident) => (if cfg!(debug_assertions){
        dbc_panic!("INVARIANT", $obj.invariant(), $obj)
    });
    ($obj:ident, $($args:tt)*) => (if cfg!(debug_assertions){
        dbc_panic!("INVARIANT", $obj.invariant(), $obj, $($args)*)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Rectangle {
        length: i32,
        width: i32,
    }

    impl Invariant for Rectangle {
        fn invariant(&self) -> bool {
            self.length > 0 && self.width > 0
        }
    }


    #[test]
    #[should_panic]
    fn test_require_asserts() {
        require!(false);
    }

    #[test]
    fn test_require_does_not_assert() {
        require!(true);
    }

    #[test]
    #[should_panic]
    fn test_ensure_asserts() {
        ensure!(false);
    }

    #[test]
    fn test_ensure_does_not_assert() {
        ensure!(true);
    }

    #[test]
    fn test_formatvar() {
        #[derive(Debug)]
        struct AA(i32);

        #[derive(Debug)]
        struct BB(AA);

        let a = 34;
        let b = BB(AA(234));
        let msg = "My message";

        assert!(formatvar!(a) == "a=34");
        assert!(formatvar!(b) == "b=BB(AA(234))");
        assert!(formatvar!(msg) == "msg=\"My message\"");
        assert!(formatvar!(msg,a,b) == "msg=\"My message\" a=34 b=BB(AA(234))");
    }

    #[test]
    fn test_invariant() {
        let r = Rectangle{
            length: 100,
            width: 30,
        };

        invariant!(r);
    }

    #[test]
    #[should_panic]
    fn test_invariant_asserts() {
        let r = Rectangle{
            length: 100,
            width: 0,
        };

        invariant!(r);
    }
}
