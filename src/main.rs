mod prom;
mod protocols;
mod sensors;

#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::State;

use prometheus::{Encoder, TextEncoder};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/metrics")]
fn metrics() -> String {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    format!("{}", String::from_utf8(buffer).unwrap())
}

#[post("/data/report", data = "<report>")]
fn ecowitt_report(
    report: Form<protocols::ecowitt::Report<'_>>,
    promobj: &State<prom::PromObjects>,
) {
    sensors::base::update_sensor_gauges(&report, promobj);
    sensors::indoor::update_sensor_gauges(&report, promobj);
    sensors::outdoor_combined::update_sensor_gauges(&report, promobj);
    sensors::raingauge::update_sensor_gauges(&report, promobj);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(prom::new())
        .mount("/", routes![index, metrics, ecowitt_report])
}
