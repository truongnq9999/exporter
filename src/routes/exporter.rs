use std::borrow::Borrow;

use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use serde_json::Value;
use wkhtmlapp::WkhtmlInput;

#[tracing::instrument(
name = "Adding a new subscriber",
skip(handlebars),
)]
pub async fn export(
    data: web::Json<Value>,
    handlebars: web::Data<Handlebars<'_>>,
    wk_app: web::Data<wkhtmlapp::App>,
) -> HttpResponse {

    let response = handlebars.render("mau9", data.into_inner().borrow()).unwrap();

    // current millis
    let pdf_path
        = wk_app.pdf_app.run(WkhtmlInput::Html(response.as_str()), "test").unwrap();

    HttpResponse::Ok().body(pdf_path)
}
