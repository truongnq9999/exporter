use crate::routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use handlebars::Handlebars;
use headless_chrome::Browser;
use tracing_actix_web::TracingLogger;

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(
    listener: TcpListener,
    // db_pool: PgPool,
    // email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "/home/truongnq/project/rust/exporter/templates/")
        .unwrap();
    let handlebars = web::Data::new(handlebars);

    let mut wk_app = wkhtmlapp::App::new().expect("Failed to init PDF application");
    let wk_app = web::Data::new(wk_app);

    let browser = Browser::default().expect("Failed to init browser");
    let browser = web::Data::new(browser);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(routes::health_check))
            .route("/export", web::post().to(routes::export))
            .route("/export2", web::post().to(routes::export2))
            .app_data(handlebars.clone())
            .app_data(wk_app.clone())
            .app_data(browser.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}
