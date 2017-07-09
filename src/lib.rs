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

//! This is comment for the module itself
//! Here too.
//!
//! # Chapter
//! *another*
//!
//! `what about this`

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
        println!("vars:\n{}", sv!($($args)*));
        env::set_var("RUST_BACKTRACE", "1");
        assert!($cond);
    })
}

/// This is a comment
#[macro_export]
macro_rules! sv {
    ($var:ident) => (format!("   {}={:?}\n", stringify!($var), $var));
    ($var:ident, $($arg:tt)*) => (format!("{}{}", sv!($var), sv!($($arg)*)));
}

/// Require asserts a condition is true.
///
/// For example, you can define a Fibonacci sequence iterator like so:
/// 
/// ```
/// #![feature(phase)]
/// #[phase(plugin)] extern crate dbc;
/// fn main() {
///     let _ =
///     ;
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

#[macro_export]
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
}
