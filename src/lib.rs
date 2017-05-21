#![feature(asm)]

extern crate core;

/// Module that contains CPUID instruction-related objects.
pub mod cpuid;

/// Module that contains operations related to Model Specific Registers.
pub mod msr;

/// Functions to send data through the processor ports.
pub mod port;

/// Segment registers.
pub mod seg;
