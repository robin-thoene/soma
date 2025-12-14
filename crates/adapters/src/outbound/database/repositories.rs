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
        let mut query = sqlx::query(
            r#"
                INSERT INTO measurements ( utc_time, weight_kg, body_fat_pct, muscle_mass_pct )
                VALUES ( ?1, ?2, ?3, ?4 )
            "#,
        )
        .bind(timestamp.to_rfc3339())
        .bind(format!("{:.2}", weight_kg));
        if let Some(x) = body_fat_pct {
            query = query.bind(format!("{:.2}", x));
        } else {
            query = query.bind(body_fat_pct);
        };
        if let Some(x) = muscle_mass_pct {
            query = query.bind(format!("{:.2}", x));
        } else {
            query = query.bind(muscle_mass_pct);
        };
        let result = query.execute(&self.db_conn).await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()), // TODO: better error handling
        }
    }
}
