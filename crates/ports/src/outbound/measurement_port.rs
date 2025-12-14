use async_trait::async_trait;

#[async_trait]
pub trait MeasurementPort: Send + Sync {
    async fn create_measurement(
        &self,
        weight_kg: f32,
        body_fat_pct: Option<f32>,
        muscle_mass_pct: Option<f32>,
    ) -> Result<(), String>; // TODO: better error types/handling
}
