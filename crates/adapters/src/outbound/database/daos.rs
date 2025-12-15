use chrono::{DateTime, Utc};
use domain::models::Measurement;
use sqlx::prelude::FromRow;

#[derive(FromRow, Clone)]
pub struct MeasurementDao {
    id: i32,
    utc_time_millis: i64,
    weight_kg: f32,
    body_fat_pct: Option<f32>,
    muscle_mass_pct: Option<f32>,
}

impl From<&MeasurementDao> for Measurement {
    fn from(value: &MeasurementDao) -> Self {
        // TODO: handle gracefully
        let utc_time: DateTime<Utc> =
            DateTime::from_timestamp_millis(value.utc_time_millis).unwrap();
        Self::new(
            value.id,
            utc_time,
            value.weight_kg,
            value.body_fat_pct,
            value.muscle_mass_pct,
        )
    }
}
