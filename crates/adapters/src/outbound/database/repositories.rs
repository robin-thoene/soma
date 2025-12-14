use domain::models::Measurement;
use ports::outbound::measurement_port::MeasurementPort;
use sqlx::SqlitePool;

pub struct MeasurementRepository {
    _db_conn: SqlitePool,
}

impl MeasurementRepository {
    pub fn new(db_conn: SqlitePool) -> Self {
        Self { _db_conn: db_conn }
    }
}

impl MeasurementPort for MeasurementRepository {
    fn create_measurement(
        weight_kg: f32,
        body_fat_pct: Option<f32>,
        muscle_mass_pct: Option<f32>,
    ) -> Result<Measurement, String> {
        // TODO: implement and remove debug fragments
        println!(
            "Insert new measurement with {:?}, {:?}, {:?}",
            weight_kg, body_fat_pct, muscle_mass_pct
        );
        todo!()
    }
}
