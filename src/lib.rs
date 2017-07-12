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

//! # Design-by-Contract for Go
//!
//! Design by Contract is a programming methodology
//! which binds the caller and the function called to a
//! contract. The contract is represented using Hoare Triple:
//! `{P} C {Q}`, where `{P}` is the precondition before
//! executing command `C`, and `{Q}` is the postcondition.
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

#[macro_export]
macro_rules! require {
    ($cond:expr) => (if cfg!(debug_assertions) {
        dbc_panic!("REQUIRE", $cond)
    });
    ($cond:expr, $($args:tt)*) => (if cfg!(debug_assertions) {
        dbc_panic!("REQUIRE", $cond, $($args)*)
    })
}

#[macro_export]
/// The entry point for panic of Rust threads.
///
/// This macro is used to inject panic into a Rust thread, causing the thread to
/// panic entirely. Each thread's panic can be reaped as the `Box<Any>` type,
/// and the single-argument form of the `panic!` macro will be the value which
/// is transmitted.
///
/// The multi-argument form of this macro panics with a string and has the
/// `format!` syntax for building a string.
///
/// # Examples
///
/// ```should_panic
/// # #![allow(unreachable_code)]
/// panic!();
/// panic!("this is a terrible mistake!");
/// panic!(4); // panic with the value of 4 to be collected elsewhere
/// panic!("this is a {} {message}", "fancy", message = "message");
/// ```
macro_rules! ensure {
    ($cond:expr) => (if cfg!(debug_assertions) {
        dbc_panic!("ENSURE", $cond)
    });
    ($cond:expr, $($args:tt)*) => (if cfg!(debug_assertions) {
        dbc_panic!("ENSURE", $cond, $($args)*)
    })
}

#[cfg(test)]
mod tests {
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
}
