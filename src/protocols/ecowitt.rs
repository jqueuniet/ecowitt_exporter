#[derive(FromForm)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Report<'r> {
    // Basic station metrics, always returned
    pub PASSKEY: &'r str,     // authentication key for ecowitt
    pub stationtype: &'r str, // general type of weather station
    pub dateutc: &'r str,     // UTC timestamp
    pub freq: &'r str,        // frequency used for wireless sensors
    pub model: &'r str,       // precise weather station model and firmware version

    // Metrics provided by indoor sensor (WH25)
    pub tempinf: Option<f64>,    // interior temperature in fahrenheit
    pub humidityin: Option<i64>, // interior humidity in percents
    pub baromrelin: Option<f64>, // interior relative pressure in in/Hg
    pub baromabsin: Option<f64>, // interior absolute pressure in in/Hg
    pub wh25batt: Option<f64>,   // WH25 sensor battery level

    // Metrics provided by outdoor 1-in-6 combined sensor (WH80)
    pub tempf: Option<f64>,          // exterior temperature in fahrenheit
    pub humidity: Option<i64>,       // exterior humidity in percents
    pub winddir: Option<i64>,        // wind direction in degrees (north as 0)
    pub windspeedmph: Option<f64>,   // average wind speed in miles/hour
    pub windgustmph: Option<f64>,    // guest wind speed in miles/hour
    pub maxdailygust: Option<f64>,   // maximum daily gust speed in miles/hour
    pub solarradiation: Option<f64>, // solar radiation in watt/m2
    pub uv: Option<i64>,             // UV index
    pub wh80batt: Option<f64>,       // WH80 sensor battery level

    // Metrics provided by outdoor rain gauge (WH40)
    pub rainratein: Option<f64>,    // Current rainfall in inch/h
    pub eventrainin: Option<f64>,   // Event rainfall in inch
    pub hourlyrainin: Option<f64>,  // Hourly rainfall in inch
    pub dailyrainin: Option<f64>,   // Daily rainfall in inch
    pub weeklyrainin: Option<f64>,  // Weekly rainfall in inch
    pub monthlyrainin: Option<f64>, // Monthly rainfall in inch
    pub yearlyrainin: Option<f64>,  // Yearly rainfall in inch
    pub wh40batt: Option<f64>,      // WH40 sensor battery level
}
