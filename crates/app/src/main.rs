use adapters::{
    inbound::mobile_app::app::App,
    outbound::database::{connection, repositories::MeasurementRepository},
};
use application::create_measurement_usecase::CreateMeasurementUsecase;
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_connection = connection::init_db_connection()
        .await
        .expect("expect db initialization to work");
    let measurement_repo = MeasurementRepository::new(db_connection);
    let create_measurment_usecase = CreateMeasurementUsecase::new(measurement_repo);
    let app = App::new(create_measurment_usecase);
    app.run()?;
    Ok(())
}
