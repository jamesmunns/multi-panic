#![no_main]
#![no_std]

use core::{panic::PanicInfo, sync::atomic::{AtomicU32, Ordering, compiler_fence}};

use defmt_rtt as _; // global logger

use nrf52840_hal as _; // memory layout

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
// #[defmt::panic_handler]
// fn panic() -> ! {
//     cortex_m::asm::udf()
// }

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}

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
