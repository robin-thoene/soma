use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_submit(move |weight, body_fat, muscle_mass| {
        println!("{}", weight);
        println!("{}", body_fat);
        println!("{}", muscle_mass);
    });

    ui.run()?;

    Ok(())
}
