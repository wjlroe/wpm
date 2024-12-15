#![windows_subsystem = "windows"]

use clap;
use std::error::Error;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};
use wpm::{config, storage, App};

fn run_gui() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let config = config::Config::new();
    let mut app = App::new(&event_loop, config);
    app.run(&mut event_loop)?;

    Ok(())
}

fn print_results() -> Result<(), Box<dyn Error>> {
    match storage::read_results_from_file() {
        Err(error) => {
            println!("{:?}", error);
            println!("source: {:?}", error.source());
        }
        Ok(results) => {
            for typing_result in results.results {
                println!("{}", typing_result);
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = clap::App::new("wpm")
        .subcommand(clap::SubCommand::with_name("results"))
        .get_matches();

    if let Some(_) = args.subcommand_matches("results") {
        print_results()
    } else {
        run_gui()
    }
}
