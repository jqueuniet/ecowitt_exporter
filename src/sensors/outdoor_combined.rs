use rocket::form::Form;
use rocket::State;

use crate::prom::PromObjects;
use crate::protocols::ecowitt::Report;
use crate::sensors::utils::{speed_mph_to_kmph, temp_f_to_c};

pub fn update_sensor_gauges(report: &Form<Report<'_>>, prom: &State<PromObjects>) {
    // Temperature
    match report.tempf {
        Some(temp) => {
            prom.temperature_f
                .with_label_values(&["exterior"])
                .set(temp);
            prom.temperature_c
                .with_label_values(&["exterior"])
                .set(temp_f_to_c(temp));
        }
        _ => {
            prom.temperature_f
                .remove_label_values(&["exterior"])
                .unwrap_or(());
            prom.temperature_c
                .remove_label_values(&["exterior"])
                .unwrap_or(());
        }
    };

    // Humidity
    match report.humidity {
        Some(hum) => {
            prom.humidity.with_label_values(&["exterior"]).set(hum);
        }
        _ => {
            prom.humidity
                .remove_label_values(&["exterior"])
                .unwrap_or(());
        }
    };

    // Wind
    match report.winddir {
        Some(dir) => {
            prom.wind_direction
                .with_label_values(&["exterior"])
                .set(dir);
        }
        _ => prom.wind_direction.reset(),
    };
    match report.windspeedmph {
        Some(speed) => {
            prom.wind_speed_mph
                .with_label_values(&["average"])
                .set(speed);
            prom.wind_speed_kmph
                .with_label_values(&["average"])
                .set(speed_mph_to_kmph(speed));
        }
        _ => {
            prom.wind_speed_mph
                .remove_label_values(&["average"])
                .unwrap_or(());
            prom.wind_speed_kmph
                .remove_label_values(&["average"])
                .unwrap_or(());
        }
    };
    match report.windgustmph {
        Some(speed) => {
            prom.wind_speed_mph.with_label_values(&["gust"]).set(speed);
            prom.wind_speed_kmph
                .with_label_values(&["gust"])
                .set(speed_mph_to_kmph(speed));
        }
        _ => {
            prom.wind_speed_mph
                .remove_label_values(&["gust"])
                .unwrap_or(());
            prom.wind_speed_kmph
                .remove_label_values(&["gust"])
                .unwrap_or(());
        }
    };
    match report.maxdailygust {
        Some(speed) => {
            prom.wind_speed_mph
                .with_label_values(&["max_daily_gust"])
                .set(speed);
            prom.wind_speed_kmph
                .with_label_values(&["max_daily_gust"])
                .set(speed_mph_to_kmph(speed));
        }
        _ => {
            prom.wind_speed_mph
                .remove_label_values(&["max_daily_gust"])
                .unwrap_or(());
            prom.wind_speed_kmph
                .remove_label_values(&["max_daily_gust"])
                .unwrap_or(());
        }
    };

    // Solar radiation
    match report.solarradiation {
        Some(rad) => {
            prom.solar_radiation_wm2
                .with_label_values(&["exterior"])
                .set(rad);
        }
        _ => {
            prom.solar_radiation_wm2.reset();
        }
    };
    match report.uv {
        Some(index) => {
            prom.uv_index.with_label_values(&["exterior"]).set(index);
        }
        _ => {
            prom.uv_index.reset();
        }
    };

    // Battery
    match report.wh80batt {
        Some(batt) => {
            prom.battery.with_label_values(&["wh80"]).set(batt);
        }
        _ => {
            prom.battery.remove_label_values(&["wh80"]).unwrap_or(());
        }
    };
}
