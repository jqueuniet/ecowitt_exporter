#[derive(FromForm)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Report<'r> {
    // Basic station metrics, always returned
    // authentication key for ecowitt
    pub PASSKEY: &'r str,
    // general type of weather station
    pub stationtype: &'r str,
    // UTC timestamp
    pub dateutc: &'r str,
    // frequency used for wireless sensors
    pub freq: &'r str,
    // precise weather station model and firmware version
    pub model: &'r str,

    // Metrics provided by indoor sensor (WH25)
    // indoor temperature in fahrenheit
    pub tempinf: Option<f64>,
    // indoor humidity in percents
    pub humidityin: Option<i64>,
    // indoor relative pressure in in/Hg
    pub baromrelin: Option<f64>,
    // indoor absolute pressure in in/Hg
    pub baromabsin: Option<f64>,
    // WH25 sensor battery level
    pub wh25batt: Option<f64>,

    // Metrics provided by outdoor 1-in-6 combined sensor (WH80)
    // outdoor temperature in fahrenheit
    pub tempf: Option<f64>,
    // outdoor humidity in percents
    pub humidity: Option<i64>,
    // wind direction in degrees (north as 0)
    pub winddir: Option<i64>,
    // average wind speed in miles/hour
    pub windspeedmph: Option<f64>,
    // guest wind speed in miles/hour
    pub windgustmph: Option<f64>,
    // maximum daily gust speed in miles/hour
    pub maxdailygust: Option<f64>,
    // solar radiation in watt/m2
    pub solarradiation: Option<f64>,
    // UV index
    pub uv: Option<i64>,
    // WH80 sensor battery level
    pub wh80batt: Option<f64>,

    // Metrics provided by outdoor rain gauge (WH40)
    // Current rainfall in inch/h
    pub rainratein: Option<f64>,
    // Event rainfall in inch
    pub eventrainin: Option<f64>,
    // Hourly rainfall in inch
    pub hourlyrainin: Option<f64>,
    // Daily rainfall in inch
    pub dailyrainin: Option<f64>,
    // Weekly rainfall in inch
    pub weeklyrainin: Option<f64>,
    // Monthly rainfall in inch
    pub monthlyrainin: Option<f64>,
    // Yearly rainfall in inch
    pub yearlyrainin: Option<f64>,
    // WH40 sensor battery level
    pub wh40batt: Option<f64>,
}
