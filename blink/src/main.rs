#![no_std]
#![no_main]

extern crate panic_halt;

use hpm6750_rs::Peripherals;

fn board_turn_off_rgb_led(dp: &Peripherals) {
    let ioc = &dp.IOC;

    ioc.pad_b[29].function.modify(|_, w| unsafe { w.alternate().bits(0) });
    ioc.pad_b[30].function.modify(|_, w| unsafe { w.alternate().bits(0) });
    // ioc.pad_b[31].function.modify(|_, w| unsafe { w.alternate().bits(0) });

    ioc.pad_b[29].pad.modify(|_, w| w.pull_enable().enable().pull_direction().down());
    ioc.pad_b[30].pad.modify(|_, w| w.pull_enable().enable().pull_direction().down());
    // ioc.pad_b[31].pad.modify(|_, w| w.pull_enable().enable().pull_direction().down());
}

fn board_init_led_pins(dp: &Peripherals) {
    let gpio0 = &dp.GPIO0;

    gpio0.oe_gpiob_set
        .write(|w| unsafe { w.bits(1 << 29 | 1 << 30) });
    gpio0.do_gpiob_set
        .write(|w| unsafe { w.bits(1 << 29 | 1 << 30) });
}

fn board_led_b_toggle(dp: &Peripherals) {
    let gpio0 = &dp.GPIO0;

    gpio0.do_gpiob_toggle.write(|w| unsafe { w.bits(1 << 29 | 1 << 30) });
}

#[no_mangle]
extern "C" fn _start() -> ! {
    let dp = Peripherals::take().unwrap();

    board_turn_off_rgb_led(&dp);
    board_init_led_pins(&dp);
    loop {
        board_led_b_toggle(&dp);
        for _ in 0..50000 {}
    }
}
