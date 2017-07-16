#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

#[macro_use]
extern crate bitflags;

pub mod runtime;
mod serial;
mod portio;
pub mod boot;
pub mod console;
pub mod bochs;
mod util;
pub mod sync;
pub mod idt;

pub mod multiboot;

// vim: set ts=2 sw=2 et :
