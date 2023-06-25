use rocket::State;
use rocket::form::Form;

pub fn update_sensor_gauges(
    report: &Form<super::super::protocols::ecowitt::Report<'_>>,
    prom: &State<super::super::prom::PromObjects>,
) {
    // ignoring date_utc as it has too much cardinality
    // and PASSKEY since it is an authentication key rather than a metric
    prom.status.reset();
    prom.status.with_label_values(&[
        report.stationtype,
        report.freq,
        report.model,
    ]).set(1.0)
}