#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_unsafe_cell_new)]
#![no_std]

#[macro_use]
extern crate bitflags;

pub mod runtime;
mod serial;
mod portio;
mod hwregs;
pub mod boot;
pub mod console;
pub mod bochs;
mod util;
pub mod sync;
pub mod idt;
pub mod multiboot;
pub mod paging;

// vim: set ts=2 sw=2 et :
