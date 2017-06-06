#![no_std]
#![crate_type = "rlib"]
#![crate_name = "asm_x86_64"]

#![feature(asm)]
#![feature(no_core)]

/// Module that contains CPUID instruction-related objects.
pub mod cpuid;

/// Module that contains operations related to Model Specific Registers.
pub mod msr;

/// Functions to send data through the processor ports.
pub mod port;

/// Segment registers.
pub mod seg;

/// Simple bitwise operations.
pub mod bit;

/// Control Register module.
pub mod cr;
