#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

pub mod runtime;
pub mod serial;
pub mod portio;
pub mod boot;
pub mod console;
pub mod bochs;
mod sync;
mod idt;

// vim: set ts=2 sw=2 et :
