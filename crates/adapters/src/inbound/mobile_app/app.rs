use crate::inbound::mobile_app::user_input_sanitizer::sanitize_weight_input;
use slint::SharedString;
use std::error::Error;

slint::include_modules!();

#[derive(Default)]
pub struct App {}

impl App {
    /// Start the application
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let ui = AppWindow::new()?;
        self.register_callbacks(&ui);
        ui.run()?;
        Ok(())
    }

    /// Registers the callbacks that are being made available through slint
    ///
    /// # Arguments
    ///
    /// * `ui` - The main application window
    fn register_callbacks(&self, ui: &AppWindow) {
        ui.on_submit(move |weight, body_fat, muscle_mass| {
            println!("{}", weight);
            println!("{}", body_fat);
            println!("{}", muscle_mass);
        });

        ui.on_compute_weight_input(move |new_val: SharedString| {
            sanitize_weight_input(new_val.as_str()).into()
        });

        ui.on_compute_body_fat_input(move |new_val: SharedString| {
            if new_val.len() > 3 {
                new_val[..3].into()
            } else {
                new_val
            }
        });

        ui.on_compute_muscle_mass_input(move |new_val: SharedString| {
            if new_val.len() > 3 {
                new_val[..3].into()
            } else {
                new_val
            }
        });
    }
}
