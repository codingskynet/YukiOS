use crate::console;
use core::{
    fmt,
    ptr::{read_volatile, write_volatile},
};

/// QEMU virt machine support 1 NS16550 compatible UART
/// https://www.qemu.org/docs/master/system/riscv/virt.html
/// https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c
struct NS16550;

const UART: usize = 0x10000000;
const UART_THR: usize = UART + 0x00;
const UART_LSR: usize = UART + 0x05;
const UART_LSR_EMPTY_MASK: u8 = 0x40;

impl fmt::Write for NS16550 {
    fn write_char(&mut self, c: char) -> fmt::Result {
        loop {
            let line_status = unsafe { read_volatile(UART_LSR as *const u8) };

            if line_status & UART_LSR_EMPTY_MASK != 0 {
                break;
            }
        }

        unsafe { write_volatile(UART_THR as *mut u8, c as u8) };

        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }
}

pub fn console() -> impl console::interface::Write {
    NS16550
}
