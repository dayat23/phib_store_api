use loco_rs::boot::{create_app, StartMode};
use loco_rs::environment::Environment;
use phib_store_api::app::App;
use migration::Migrator;

#[shuttle_runtime::main]
async fn main(
  #[shuttle_shared_db::Postgres] conn_str: String,
  #[shuttle_metadata::ShuttleMetadata] meta: shuttle_metadata::Metadata,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var("DATABASE_URL", conn_str);
    let environment = match meta.env {
        shuttle_metadata::Environment::Local => Environment::Development,
        shuttle_metadata::Environment::Deployment => Environment::Production,
    };
    let boot_result = create_app::<App, Migrator>(StartMode::ServerOnly, &environment)
        .await
        .unwrap();

    let router = boot_result.router.unwrap();
    Ok(router.into())
}
