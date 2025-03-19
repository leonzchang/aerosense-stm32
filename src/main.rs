#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_halt as _;

// RCC registers
const RCC: u32 = 0x4002_1000;
const RCC_AHB2_PERIPHERALS_CLOCK_ENABLE_REGISTER_OFFSET: u32 = 0x4C;
const RCC_AHB2_PERIPHERALS_CLOCK: *mut u32 =
    (RCC + RCC_AHB2_PERIPHERALS_CLOCK_ENABLE_REGISTER_OFFSET) as *mut u32;

// Bus addresses
const GPIOB_BUS_ADDRESS: u32 = 0x4800_0400;

// GPIO registers offset
const GPIOX_MODER_OFFSET: u32 = 0x00; // GPIO Mode
const GPIOX_BSRR_OFFSET: u32 = 0x18; // GPIO set/reset register

// GPIO registers pointers for MMIO
const GPIOB_MODER: *mut u32 = (GPIOB_BUS_ADDRESS + GPIOX_MODER_OFFSET) as *mut u32;
const GPIOB_BSRR: *mut u32 = (GPIOB_BUS_ADDRESS + GPIOX_BSRR_OFFSET) as *mut u32;

#[entry]
fn main() -> ! {
    unsafe {
        // Enable the GPIO peripheral clock
        let value = RCC_AHB2_PERIPHERALS_CLOCK.read_volatile();
        RCC_AHB2_PERIPHERALS_CLOCK.write_volatile(value | (1 << 1));

        // Set the User LED GPIOB to output mode(MODE3 6, 7 bit)
        let mut moder = GPIOB_MODER.read_volatile();
        moder &= !(0b11 << (3 * 2)); // Clear bits 6 and 7 (3 * 2)
        moder |= 0b01 << (3 * 2); // Set bits 6 and 7 to 01
        GPIOB_MODER.write_volatile(moder);

        loop {
            // The code can be executed from SRAM or from flash, and the CPU frequency is limited to 2 MHz.
            // 10_000_000 / 2_000_000 = 5 seconds
            GPIOB_BSRR.write_volatile(1 << 3);
            delay(10_000_000);

            GPIOB_BSRR.write_volatile(1 << (19));
            delay(10_000_000);
        }
    }
}
