CREATE TABLE IF NOT EXISTS measurements
(
    id                  INTEGER PRIMARY KEY NOT NULL,
    utc_time            TEXT                NOT NULL,
    weight_kg           REAL                NOT NULL,
    body_fat_pct        REAL,
    muscle_mass_pct     REAL
);
