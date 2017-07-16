
[![Build Status](https://travis-ci.org/lpabon/rust-dbc.svg?branch=master)](https://travis-ci.org/lpabon/rust-dbc)

# rust-dbc
Simple design by contract macros for Rust

Please see the [Documentation](https://docs.rs/dbc/0.3.3/) or [example](example/src/main.rs) for more information.

# Example

Here is a simple `require!` example:

```rust
    let msg = "This is a test";
    let a = 3;
    require!(false, msg, a);
```

Outputs:

```
panic: REQUIRE:
file: src/main.rs:45
vars:
msg="This is a test" a=3
thread 'main' panicked at 'assertion failed: false', src/main.rs:45
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:371
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:511
   6: example::main
             at ./src/main.rs:45
   7: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
   8: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:433
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:57
   9: main
  10: __libc_start_main
  11: _start
```
