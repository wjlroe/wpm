use glutin::EventsLoop;
use std::error::Error;
use wpm::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut event_loop = EventsLoop::new();
    let mut app = App::new(&event_loop);
    app.run(&mut event_loop)?;

    Ok(())
}
