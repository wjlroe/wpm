use clap;
use glutin::EventsLoop;
use std::error::Error;
use wpm::storage;
use wpm::App;

fn run_gui() -> Result<(), Box<dyn Error>> {
    let mut event_loop = EventsLoop::new();
    let mut app = App::new(&event_loop);
    app.run(&mut event_loop)?;

    Ok(())
}

fn print_results() -> Result<(), Box<dyn Error>> {
    let results = storage::read_results_from_file()?;
    for typing_result in results.results {
        println!("{}", typing_result);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = clap::App::new("wpm")
        .subcommand(clap::SubCommand::with_name("results"))
        .get_matches();

    if let Some(_) = args.subcommand_matches("results") {
        print_results()
    } else {
        run_gui()
    }
}
