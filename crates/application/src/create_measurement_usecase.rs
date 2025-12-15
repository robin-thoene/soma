use async_trait::async_trait;
use ports::{
    inbound::create_measurement_port::CreateMeasurementPort,
    outbound::measurement_port::MeasurementPort,
};
use std::sync::Arc;

pub struct CreateMeasurementUsecase<M>
where
    M: MeasurementPort,
{
    measurement_port: Arc<M>,
}

impl<M> CreateMeasurementUsecase<M>
where
    M: MeasurementPort,
{
    pub fn new(measurement_port: Arc<M>) -> Self {
        Self { measurement_port }
    }
}

#[async_trait]
impl<M> CreateMeasurementPort for CreateMeasurementUsecase<M>
where
    M: MeasurementPort,
{
    async fn create(
        &self,
        weight_kg: f32,
        body_fat_pct: Option<f32>,
        muscle_mass_pct: Option<f32>,
    ) -> Result<(), String> {
        self.measurement_port
            .create_measurement(weight_kg, body_fat_pct, muscle_mass_pct)
            .await
    }
}
