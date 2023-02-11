//to run main
//cd to this dir and hit "cargo run"

#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window,WindowBuilder}
};

fn main() {
    //println!("Hello, world with cargo!");
    logger("Begin Program".to_string());
    logger("Eng Program".to_string());
}


fn logger(message : String){
    println!("{}", message);
}