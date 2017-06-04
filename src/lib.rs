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
mod util;
mod sync;
mod idt;

mod idt_gen {
  include!(concat!(env!("OUT_DIR"), "/idt_gen.rs"));
}

// vim: set ts=2 sw=2 et :
