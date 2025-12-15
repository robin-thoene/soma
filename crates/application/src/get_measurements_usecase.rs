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
    _measurement_port: Arc<M>,
}

impl<M> GetMeasurementsUsecase<M>
where
    M: MeasurementPort,
{
    pub fn new(measurement_port: Arc<M>) -> Self {
        Self {
            _measurement_port: measurement_port,
        }
    }
}

#[async_trait]
impl<M> GetMeasurementsPort for GetMeasurementsUsecase<M>
where
    M: MeasurementPort,
{
    async fn get(&self) -> Result<Vec<Measurement>, String> {
        Ok(vec![])
    }
}
