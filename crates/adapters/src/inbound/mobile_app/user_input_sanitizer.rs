/// The allowed maximum value for the weight user input in kg
const MAX_WEIGHT_IN_KG: f32 = 300.0;
/// The allowed minimum value for the weight user input in kg
const MIN_WEIGHT_IN_KG: f32 = 0.00;

pub fn sanitize_weight_input(input: &str) -> String {
    // TODO: refactor once covered by tests
    match input.trim().parse::<f32>() {
        Ok(num) => {
            if num > MAX_WEIGHT_IN_KG {
                return format!("{:.2}", MAX_WEIGHT_IN_KG);
            }
            if num <= MIN_WEIGHT_IN_KG {
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
    "".into()
}

pub fn sanitize_muscle_mass_input(input: &str) -> String {
    "".into()
}
