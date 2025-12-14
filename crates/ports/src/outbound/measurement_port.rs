use domain::models::Measurement;

pub trait MeasurementPort {
    fn create_measurement(
        weight_kg: f32,
        body_fat_pct: Option<f32>,
        muscle_mass_pct: Option<f32>,
    ) -> Result<Measurement, String>; // TODO: better error types/handling
}
