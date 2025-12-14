use async_trait::async_trait;
use chrono::Utc;
use ports::outbound::measurement_port::MeasurementPort;
use sqlx::SqlitePool;

pub struct MeasurementRepository {
    db_conn: SqlitePool,
}

impl MeasurementRepository {
    pub fn new(db_conn: SqlitePool) -> Self {
        Self { db_conn }
    }
}

#[async_trait]
impl MeasurementPort for MeasurementRepository {
    async fn create_measurement(
        &self,
        weight_kg: f32,
        body_fat_pct: Option<f32>,
        muscle_mass_pct: Option<f32>,
    ) -> Result<(), String> {
        let timestamp = Utc::now();
        let result = sqlx::query(
            r#"
                INSERT INTO todos ( measurements )
                VALUES ( ?1, ?2, ?3, ?4 )
            "#,
        )
        .bind(timestamp.to_rfc3339())
        .bind(weight_kg)
        .bind(body_fat_pct)
        .bind(muscle_mass_pct)
        .execute(&self.db_conn)
        .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()), // TODO: better error handling
        }
    }
}
