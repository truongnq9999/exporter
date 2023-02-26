use std::borrow::Borrow;
use std::fs::File;

use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use headless_chrome::Browser;
use serde_json::Value;
use uuid::Uuid;
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
    let pdf_path = wk_app.pdf_app.run(WkhtmlInput::Html(response.as_str()), "test").unwrap();

    println!("generated PDF saved as: f");
    println!("{}", pdf_path);

    HttpResponse::Ok().body(pdf_path)
}


#[tracing::instrument(
name = "export2",
skip(handlebars, browser),
)]
pub async fn export2(
    data: web::Json<Value>,
    handlebars: web::Data<Handlebars<'_>>,
    browser: web::Data<Browser>,
) -> HttpResponse {
    let response = handlebars.render("mau9", data.into_inner().borrow()).unwrap();

    let html_path = format!("/tmp/exporter/{}.html", Uuid::new_v4());
    std::fs::write(html_path.clone(), response).unwrap();

    let html_url = format!("file://{}", html_path);
    let tab = browser.new_tab().unwrap();
    tab.navigate_to(html_url.as_str()).unwrap();
    tab.wait_until_navigated().unwrap();
    tab.wait_for_element("body").unwrap();

    let pdf_data = tab.print_to_pdf(None).unwrap();
    let pdf_path = format!("/tmp/exporter/{}.pdf", Uuid::new_v4());

    std::fs::write(pdf_path.clone(), pdf_data).unwrap();

    HttpResponse::Ok().body(pdf_path)
}