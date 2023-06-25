use prometheus::core::{AtomicF64, GenericGaugeVec};
use prometheus::{opts, register_gauge_vec, register_int_gauge_vec, IntGaugeVec};

pub struct PromObjects {
    // Only Vec variants of gauges are used since the others can't be easily removed
    // Removing gauges is important for cases where sensors disapear (ie, exhausted battery), in
    // order to stop sending stale metrics during scrape
    // A single dummy sensor label will be used for those situations

    // Basic status Gauge
    pub status: GenericGaugeVec<AtomicF64>,

    // Common generic gauges
    pub temperature_f: GenericGaugeVec<AtomicF64>,
    pub temperature_c: GenericGaugeVec<AtomicF64>,
    pub pressure_inhg: GenericGaugeVec<AtomicF64>,
    pub pressure_pa: GenericGaugeVec<AtomicF64>,
    pub humidity: IntGaugeVec,
    pub battery: GenericGaugeVec<AtomicF64>,

    // Exterior wind gauges
    pub wind_direction: IntGaugeVec,
    pub wind_speed_mph: GenericGaugeVec<AtomicF64>,
    pub wind_speed_kmph: GenericGaugeVec<AtomicF64>,

    // Exterior solar gauges
    pub solar_radiation_wm2: GenericGaugeVec<AtomicF64>,
    pub uv_index: IntGaugeVec,

    // Exterior rain gauges
    pub rainfall_rate_inh: GenericGaugeVec<AtomicF64>,
    pub rainfall_rate_mmh: GenericGaugeVec<AtomicF64>,
    pub rainfall_in: GenericGaugeVec<AtomicF64>,
    pub rainfall_mm: GenericGaugeVec<AtomicF64>,
}

pub fn new() -> PromObjects {
    PromObjects {
        status: register_gauge_vec!(
            opts!("ecowitt_status", "status infos fronm weather station",),
            &["type", "freq", "model"]
        )
        .unwrap(),

        temperature_c: register_gauge_vec!(
            opts!(
                "ecowitt_temperature_celsius",
                "temperature measurements (celsius)",
            ),
            &["sensor"]
        )
        .unwrap(),
        temperature_f: register_gauge_vec!(
            opts!(
                "ecowitt_temperature_fahrenheit",
                "temperature measurements (fahrenheit)",
            ),
            &["sensor"]
        )
        .unwrap(),
        pressure_pa: register_gauge_vec!(
            opts!("ecowitt_pressure_pascal", "pressure measurements (Pa)",),
            &["sensor", "type"]
        )
        .unwrap(),
        pressure_inhg: register_gauge_vec!(
            opts!(
                "ecowitt_pressure_inchesofmercury",
                "pressure measurements (inHg)",
            ),
            &["sensor", "type"]
        )
        .unwrap(),
        humidity: register_int_gauge_vec!(
            opts!(
                "ecowitt_humidity_percents",
                "humidity measurements (percents)",
            ),
            &["sensor"]
        )
        .unwrap(),
        battery: register_gauge_vec!(
            opts!("ecowitt_battery_level", "battery measurements",),
            &["sensor"]
        )
        .unwrap(),

        wind_direction: register_int_gauge_vec!(
            opts!("ecowitt_wind_direction_degrees", "wind direction (degrees)",),
            &["sensor"]
        )
        .unwrap(),
        wind_speed_kmph: register_gauge_vec!(
            opts!(
                "ecowitt_wind_speed_kilometersperhour",
                "wind speed (km/hour)",
            ),
            &["type"]
        )
        .unwrap(),
        wind_speed_mph: register_gauge_vec!(
            opts!("ecowitt_wind_speed_milesperhour", "wind speed (miles/hour)",),
            &["type"]
        )
        .unwrap(),

        solar_radiation_wm2: register_gauge_vec!(
            opts!(
                "ecowitt_solar_radiation_wattspersquaremeter",
                "solar radiation (watts/square meter)",
            ),
            &["sensor"]
        )
        .unwrap(),
        uv_index: register_int_gauge_vec!(opts!("ecowitt_uv_index", "UV index",), &["sensor"])
            .unwrap(),

        rainfall_rate_inh: register_gauge_vec!(
            opts!(
                "ecowitt_rainfall_rate_inchesperhour",
                "current rainfall rate (inches/hour)",
            ),
            &["sensor"]
        )
        .unwrap(),
        rainfall_rate_mmh: register_gauge_vec!(
            opts!(
                "ecowitt_rainfall_rate_millimetersperhour",
                "current rainfall rate (millimeters/hour)",
            ),
            &["sensor"]
        )
        .unwrap(),
        rainfall_in: register_gauge_vec!(
            opts!("ecowitt_rainfall_inches", "rainfall depth (inches)",),
            &["period"]
        )
        .unwrap(),
        rainfall_mm: register_gauge_vec!(
            opts!(
                "ecowitt_rainfall_millimeters",
                "rainfall depth (millimeters)",
            ),
            &["period"]
        )
        .unwrap(),
    }
}
