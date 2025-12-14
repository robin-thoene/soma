use crate::inbound::mobile_app::user_input_sanitizer::{
    sanitize_body_fat_input, sanitize_muscle_mass_input, sanitize_weight_input,
};
use ports::inbound::create_measurement_port::CreateMeasurementPort;
use slint::SharedString;
use std::{error::Error, sync::Arc};

slint::include_modules!();

// TODO: find a way to do this without introducing a static lifetime

pub struct App<C>
where
    C: CreateMeasurementPort + 'static,
{
    create_measurement_port: Arc<C>,
}

impl<C> App<C>
where
    C: CreateMeasurementPort + 'static,
{
    pub fn new(create_measurement_port: C) -> Self {
        Self {
            create_measurement_port: Arc::new(create_measurement_port),
        }
    }

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
        let port = Arc::clone(&self.create_measurement_port);

        ui.on_submit(move |weight, body_fat, muscle_mass| {
            // TODO: replace with safe way of parsing
            let weight_kg = weight.trim().parse::<f32>().unwrap();
            let body_fat_pct = if body_fat.is_empty() {
                None
            } else {
                Some(body_fat.trim().parse::<f32>().unwrap())
            };
            let muscle_mass_pct = if muscle_mass.is_empty() {
                None
            } else {
                Some(muscle_mass.trim().parse::<f32>().unwrap())
            };

            let port = Arc::clone(&port);

            let _ = slint::spawn_local(async move {
                let creation_result = port.create(weight_kg, body_fat_pct, muscle_mass_pct).await;
                println!("{:?}", creation_result);
            });
        });

        ui.on_compute_weight_input(move |new_val: SharedString| {
            sanitize_weight_input(new_val.as_str()).into()
        });

        ui.on_compute_body_fat_input(move |new_val: SharedString| {
            sanitize_body_fat_input(new_val.as_str()).into()
        });

        ui.on_compute_muscle_mass_input(move |new_val: SharedString| {
            sanitize_muscle_mass_input(new_val.as_str()).into()
        });
    }
}
