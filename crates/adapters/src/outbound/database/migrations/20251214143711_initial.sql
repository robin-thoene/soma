CREATE TABLE IF NOT EXISTS measurements
(
    id                  INTEGER PRIMARY KEY NOT NULL,
    utc_time_millis     INTEGER             NOT NULL,
    weight_kg           REAL                NOT NULL,
    body_fat_pct        REAL,
    muscle_mass_pct     REAL
);

CREATE INDEX IF NOT EXISTS idx_measurements_utc_time_millis
ON measurements (utc_time_millis DESC);
