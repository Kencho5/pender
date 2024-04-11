mod app_state;
mod config;
mod imports;
mod register_routes;
mod routes;
mod utils;
use crate::imports::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config = config::config_manager::load_config().expect("Config Not Found.");
    // tide::log::start();

    let mut tera = Tera::new("./templates/**/*")?;
    tera.autoescape_on(vec!["html"]);
    let translations = utils::translations::load_translations().await.unwrap();

    let mut app = tide::with_state(AppState { tera, translations });

    let secret_key =
        std::env::var("TIDE_SECRET").expect("TIDE_SECRET environment variable not set");
    app.with(tide::sessions::SessionMiddleware::new(
        MemoryStore::new(),
        secret_key.as_bytes(),
    ));

    let connection_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &config.database.username,
        &config.database.password,
        &config.database.host,
        &config.database.port,
        &config.database.db_name
    );
    app.with(SQLxMiddleware::<Postgres>::new(&connection_url).await?);

    app.at("/assets").serve_dir("./public/assets/")?;
    app.at("/static").serve_dir("./public/static/")?;

    register_routes::register_routes(&mut app);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
