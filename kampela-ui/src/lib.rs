#![no_std]

pub mod uistate;
pub mod platform;
pub mod widget{
    pub mod button;
    pub mod view;
}

pub mod display_def;
pub mod pin;
mod seed_entry;
pub mod backup;
mod restore_or_generate;
mod test;

pub mod transaction;
pub mod qr;

#[macro_use]
extern crate lazy_static;

#[cfg(not(feature="std"))]
extern crate alloc;
#[cfg(not(feature="std"))]
extern crate core;
#[cfg(feature="std")]
extern crate std;

pub mod data_state;
