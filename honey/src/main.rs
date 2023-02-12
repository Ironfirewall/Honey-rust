//to run main
//cd to this dir and hit "cargo run"

#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

//use for reference, this should always work properly
//include!("logger/logger.rs");

mod test;

mod logger;

use test::something::runTest;

use anyhow::Result;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window,WindowBuilder}
};

fn main() -> Result<()> {

    //println!("Hello, world with cargo!");
    logger::log("Begin Program".to_string());
    runTest();
    logger::log("Eng Program".to_string());

    pretty_env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Honey 3D")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

        // App

    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !destroying =>
                unsafe { app.render(&window) }.unwrap(),
            // Destroy our Vulkan app.
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            }
            _ => {}
        }
    });
}


// fn logger(message : String){
//     println!("{}", message);
// }

/// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {}

impl App {
    /// Creates our Vulkan app.
    unsafe fn create(window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    unsafe fn destroy(&mut self) {}
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}