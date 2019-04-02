use std::error::Error;
use wpm::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    app.run()?;

    Ok(())
}
