use adapters::{
    inbound::mobile_app::app::App,
    outbound::database::{connection, repositories::MeasurementRepository},
};
use application::{
    create_measurement_usecase::CreateMeasurementUsecase,
    get_measurements_usecase::GetMeasurementsUsecase,
};
use std::{error::Error, sync::Arc};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_connection = connection::init_db_connection()
        .await
        .expect("expect db initialization to work");

    let measurement_repo = Arc::new(MeasurementRepository::new(db_connection));

    let create_measurment_usecase =
        Arc::new(CreateMeasurementUsecase::new(Arc::clone(&measurement_repo)));
    let get_measurements_usecase =
        Arc::new(GetMeasurementsUsecase::new(Arc::clone(&measurement_repo)));

    let app = App::new(create_measurment_usecase, get_measurements_usecase);
    app.run()?;
    Ok(())
}
