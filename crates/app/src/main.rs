use adapters::{
    inbound::mobile_app::app::App,
    outbound::database::{connection, repositories::MeasurementRepository},
};
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_connection = connection::init_db_connection()
        .await
        .expect("expect db initialization to work");
    let _measurement_port = MeasurementRepository::new(db_connection);
    let app = App::default();
    app.run()?;
    Ok(())
}
