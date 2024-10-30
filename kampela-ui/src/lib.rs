#![no_std]
#![deny(unused_crate_dependencies)]

pub mod uistate;
pub mod widget{
    pub mod view;
    pub mod nav_bar{
        pub mod nav_bar;
        pub mod nav_button;
    }
}

pub mod display_def;

mod message;
mod dialog;

pub mod probe {
    pub mod probe;
    pub mod graph;
    pub mod coordinates;
    pub mod labels;
}

#[macro_use]
extern crate lazy_static;

#[cfg(not(feature="std"))]
extern crate alloc;
#[cfg(not(feature="std"))]
extern crate core;
#[cfg(feature="std")]
extern crate std;

