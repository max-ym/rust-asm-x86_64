#![no_std]
#![crate_type = "rlib"]
#![crate_name = "asm_x86_64"]

#![feature(asm)]
#![feature(no_core)]

#[macro_use]
extern crate new_bitflags;

/// This module simulates 'std' library for extern crates.
mod std {
pub use core::*;
}

/// Module related to I/O APIC and Local APIC.
pub mod apic;

/// Module that contains CPUID instruction-related objects.
pub mod cpuid;

/// Module with accelerated memory operations.
pub mod mem;

/// Module that contains operations related to Model Specific Registers.
pub mod msr;

/// Functions to send data through the processor ports.
pub mod port;

/// Programable Interrupt Controller module.
pub mod pic;

/// Register files module.
pub mod regf;

/// Segment registers.
pub mod seg;

/// Module related to GDT, IDT, paging tables.
pub mod tables;
pub use tables::{gdt, idt, paging};

/// Simple bitwise operations that can be accelerated by CPU.
pub mod bit;

/// Control Register module.
pub mod cr;

/// Programmable Interval Timer.
pub mod pit;

/// XSAVE instruction module.
pub mod xsave;
