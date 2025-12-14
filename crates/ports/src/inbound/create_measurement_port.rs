use async_trait::async_trait;

#[async_trait]
pub trait CreateMeasurementPort {
    async fn create(
        &self,
        weight_kg: f32,
        body_fat_pct: Option<f32>,
        muscle_mass_pct: Option<f32>,
    ) -> Result<(), String>;
}
