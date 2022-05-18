# Oops all panics!

## The context:

https://twitter.com/Geoxion/status/1526936058788827141

This is a repro of that report. It **does confirm** that in `panic = "abort"` (at least on embedded), panics WILL recurse, rather than hard-abort on a double-panic.

## The panic handler:

```rust
static PCTR: AtomicU32 = AtomicU32::new(0);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let ct = PCTR.fetch_add(1, Ordering::Relaxed);
    defmt::println!("Panic {=u32}", ct);

    if ct < 10 {
        panic!();
    } else {
        defmt::println!("Hit recursion limit!");
        loop {
            compiler_fence(Ordering::SeqCst);
        }
    }
}
```

## The program:

```rust
fn main() -> ! {
    defmt::println!("Here we go!");

    panic!();
}
```

## The result:

```text
➜  multi-panic git:(main) ✗ cargo rrb mpanic
    Finished release [optimized + debuginfo] target(s) in 0.02s
     Running `probe-run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/mpanic`
(HOST) INFO  flashing program (2 pages / 8.00 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
Here we go!
└─ mpanic::__cortex_m_rt_main @ src/bin/mpanic.rs:8
Panic 0
└─ multi_panic::panic @ src/lib.rs:43
Panic 1
└─ multi_panic::panic @ src/lib.rs:43
Panic 2
└─ multi_panic::panic @ src/lib.rs:43
Panic 3
└─ multi_panic::panic @ src/lib.rs:43
Panic 4
└─ multi_panic::panic @ src/lib.rs:43
Panic 5
└─ multi_panic::panic @ src/lib.rs:43
Panic 6
└─ multi_panic::panic @ src/lib.rs:43
Panic 7
└─ multi_panic::panic @ src/lib.rs:43
Panic 8
└─ multi_panic::panic @ src/lib.rs:43
Panic 9
└─ multi_panic::panic @ src/lib.rs:43
Panic 10
└─ multi_panic::panic @ src/lib.rs:43
Hit recursion limit!
└─ multi_panic::panic @ src/lib.rs:48
```

Control-C is hit. This triggers probe-run to stop the program
and generate a backtrace of the current program state.

```text
────────────────────────────────────────────────────────────────────────────────
stack backtrace:
   0: rust_begin_unwind
        at src/lib.rs:49:9
   1: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
   2: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
   3: rust_begin_unwind
        at src/lib.rs:46:9
   4: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
   5: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
   6: rust_begin_unwind
        at src/lib.rs:46:9
   7: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
   8: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
   9: rust_begin_unwind
        at src/lib.rs:46:9
  10: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
  11: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
  12: rust_begin_unwind
        at src/lib.rs:46:9
  13: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
  14: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
  15: rust_begin_unwind
        at src/lib.rs:46:9
  16: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
  17: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
  18: rust_begin_unwind
        at src/lib.rs:46:9
  19: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
  20: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
  21: rust_begin_unwind
        at src/lib.rs:46:9
  22: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
  23: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
  24: rust_begin_unwind
        at src/lib.rs:46:9
  25: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
  26: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
  27: rust_begin_unwind
        at src/lib.rs:46:9
  28: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
  29: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
  30: rust_begin_unwind
        at src/lib.rs:46:9
  31: core::panicking::panic_fmt
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
  32: core::panicking::panic
        at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:48:5
  33: mpanic::__cortex_m_rt_main
        at src/bin/mpanic.rs:10:5
  34: main
        at src/bin/mpanic.rs:6:1
  35: Reset
(HOST) INFO  device halted by user
```
