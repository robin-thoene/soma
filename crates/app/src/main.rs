use adapters::inbound::mobile_app::app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::default();
    app.run()?;
    Ok(())
}
