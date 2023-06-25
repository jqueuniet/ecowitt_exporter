use rocket::State;
use rocket::form::Form;

use crate::protocols::ecowitt::Report;
use crate::prom::PromObjects;
use crate::sensors::utils::length_in_to_mm;

pub fn update_sensor_gauges(
    report: &Form<Report<'_>>,
    prom: &State<PromObjects>,
) {
    // Rain rate
    match report.rainratein {
        Some(rate) => {
            prom.rainfall_rate_inh.
                with_label_values(&["exterior"]).
                set(rate);
            prom.rainfall_rate_mmh.
                with_label_values(&["exterior"]).
                set(length_in_to_mm(rate));
        }
        _ => {
            prom.rainfall_rate_inh.reset();
            prom.rainfall_rate_mmh.reset();
        }
    };

    // Rain Depth
    match report.eventrainin {
        Some(depth) => {
            prom.rainfall_in.
                with_label_values(&["event"]).
                set(depth);
            prom.rainfall_mm.
                with_label_values(&["event"]).
                set(length_in_to_mm(depth));
        }
        _ => {
            prom.rainfall_in.
                remove_label_values(&["event"]).
                unwrap_or(());
            prom.rainfall_mm.
                remove_label_values(&["event"]).
                unwrap_or(());
        }
    };
    match report.hourlyrainin {
        Some(depth) => {
            prom.rainfall_in.
                with_label_values(&["hourly"]).
                set(depth);
            prom.rainfall_mm.
                with_label_values(&["hourly"]).
                set(length_in_to_mm(depth));
        }
        _ => {
            prom.rainfall_in.
                remove_label_values(&["hourly"]).
                unwrap_or(());
            prom.rainfall_mm.
                remove_label_values(&["hourly"]).
                unwrap_or(());
        }
    };
    match report.dailyrainin {
        Some(depth) => {
            prom.rainfall_in.
                with_label_values(&["daily"]).
                set(depth);
            prom.rainfall_mm.
                with_label_values(&["daily"]).
                set(length_in_to_mm(depth));
        }
        _ => {
            prom.rainfall_in.
                remove_label_values(&["daily"]).
                unwrap_or(());
            prom.rainfall_mm.
                remove_label_values(&["daily"]).
                unwrap_or(());
        }
    };
    match report.weeklyrainin {
        Some(depth) => {
            prom.rainfall_in.
                with_label_values(&["weekly"]).
                set(depth);
            prom.rainfall_mm.
                with_label_values(&["weekly"]).
                set(length_in_to_mm(depth));
        }
        _ => {
            prom.rainfall_in.
                remove_label_values(&["weekly"]).
                unwrap_or(());
            prom.rainfall_mm.
                remove_label_values(&["weekly"]).
                unwrap_or(());
        }
    };
    match report.monthlyrainin {
        Some(depth) => {
            prom.rainfall_in.
                with_label_values(&["monthly"]).
                set(depth);
            prom.rainfall_mm.
                with_label_values(&["monthly"]).
                set(length_in_to_mm(depth));
        }
        _ => {
            prom.rainfall_in.
                remove_label_values(&["monthly"]).
                unwrap_or(());
            prom.rainfall_mm.
                remove_label_values(&["monthly"]).
                unwrap_or(());
        }
    };
    match report.yearlyrainin {
        Some(depth) => {
            prom.rainfall_in.
                with_label_values(&["yearly"]).
                set(depth);
            prom.rainfall_mm.
                with_label_values(&["yearly"]).
                set(length_in_to_mm(depth));
        }
        _ => {
            prom.rainfall_in.
                remove_label_values(&["yearly"]).
                unwrap_or(());
            prom.rainfall_mm.
                remove_label_values(&["yearly"]).
                unwrap_or(());
        }
    };

    // Battery
    match report.wh40batt {
        Some(batt) => {
            prom.battery.
                with_label_values(&["wh40"]).
                set(batt);
        }
        _ => {
            prom.battery.
                remove_label_values(&["wh40"]).
                unwrap_or(());
        }
    };
}