// TODO: define more realistic max and min values per category
const MAX_WEIGHT_IN_KG: f32 = 300.0;
const MIN_WEIGHT_IN_KG: f32 = 0.00;
const MAX_BODY_FAT_PERCENTAGE: f32 = 100.0;
const MIN_BODY_FAT_PERCENTAGE: f32 = 0.0;
const MAX_MUSCLE_MASS_PERCENTAGE: f32 = 100.0;
const MIN_MUSCLE_MASS_PERCENTAGE: f32 = 0.0;

pub fn sanitize_weight_input(input: &str) -> String {
    // TODO: refactor once covered by tests
    match input.trim().parse::<f32>() {
        Ok(num) => {
            if num > MAX_WEIGHT_IN_KG {
                return format!("{:.2}", MAX_WEIGHT_IN_KG);
            }
            if num < MIN_WEIGHT_IN_KG {
                return "".into();
            }
            let split = input.split_once('.');
            if let Some(parts) = split {
                // TODO: refactor once covered by tests
                if parts.1.len() > 2 {
                    let end = parts.1[0..2].to_string();
                    return format!("{}.{}", parts.0, end);
                }
            }
            input.into()
        }
        Err(_err) => "".into(),
    }
}

pub fn sanitize_body_fat_input(input: &str) -> String {
    // TODO: refactor once covered by tests
    match input.trim().parse::<f32>() {
        Ok(num) => {
            if num > MAX_BODY_FAT_PERCENTAGE {
                return format!("{:.2}", MAX_BODY_FAT_PERCENTAGE);
            }
            if num < MIN_BODY_FAT_PERCENTAGE {
                return "".into();
            }
            let split = input.split_once('.');
            if let Some(parts) = split {
                // TODO: refactor once covered by tests
                if parts.1.len() > 2 {
                    let end = parts.1[0..2].to_string();
                    return format!("{}.{}", parts.0, end);
                }
            }
            input.into()
        }
        Err(_err) => "".into(),
    }
}

pub fn sanitize_muscle_mass_input(input: &str) -> String {
    // TODO: refactor once covered by tests
    match input.trim().parse::<f32>() {
        Ok(num) => {
            if num > MAX_MUSCLE_MASS_PERCENTAGE {
                return format!("{:.2}", MAX_MUSCLE_MASS_PERCENTAGE);
            }
            if num < MIN_MUSCLE_MASS_PERCENTAGE {
                return "".into();
            }
            let split = input.split_once('.');
            if let Some(parts) = split {
                // TODO: refactor once covered by tests
                if parts.1.len() > 2 {
                    let end = parts.1[0..2].to_string();
                    return format!("{}.{}", parts.0, end);
                }
            }
            input.into()
        }
        Err(_err) => "".into(),
    }
}
