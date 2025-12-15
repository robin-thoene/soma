use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Measurement {
    id: i32,
    utc_time: DateTime<Utc>,
    weight_kg: f32,
    body_fat_pct: Option<f32>,
    muscle_mass_pct: Option<f32>,
}

impl Measurement {
    pub fn new(
        id: i32,
        utc_time: DateTime<Utc>,
        weight_kg: f32,
        body_fat_pct: Option<f32>,
        muscle_mass_pct: Option<f32>,
    ) -> Self {
        Self {
            id,
            utc_time,
            weight_kg,
            body_fat_pct,
            muscle_mass_pct,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_utc_time(&self) -> DateTime<Utc> {
        self.utc_time
    }

    pub fn get_weight_kg(&self) -> f32 {
        self.weight_kg
    }

    pub fn get_body_fat_pct(&self) -> Option<f32> {
        self.body_fat_pct
    }

    pub fn get_muscle_mass_pct(&self) -> Option<f32> {
        self.muscle_mass_pct
    }
}
