use async_trait::async_trait;
use domain::models::Measurement;
use ports::{
    inbound::get_measurements_port::GetMeasurementsPort,
    outbound::measurement_port::MeasurementPort,
};
use std::sync::Arc;

pub struct GetMeasurementsUsecase<M>
where
    M: MeasurementPort,
{
    measurement_port: Arc<M>,
}

impl<M> GetMeasurementsUsecase<M>
where
    M: MeasurementPort,
{
    pub fn new(measurement_port: Arc<M>) -> Self {
        Self { measurement_port }
    }
}

#[async_trait]
impl<M> GetMeasurementsPort for GetMeasurementsUsecase<M>
where
    M: MeasurementPort,
{
    async fn get(&self) -> Result<Vec<Measurement>, String> {
        // TODO: add necessary business logic
        self.measurement_port.get_measurements().await
    }
}
