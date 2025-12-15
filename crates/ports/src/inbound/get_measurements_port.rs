use async_trait::async_trait;
use domain::models::Measurement;

#[async_trait]
pub trait GetMeasurementsPort {
    async fn get(&self) -> Result<Vec<Measurement>, String>;
}
