# ecowitt_exporter

This program listens to ecowitt protocol submissions to provide them in the Prometheus/OpenMetrics format. 

## Supported hardware

The Ecowitt protocol should be supported by the various Ecowitt clones like Ambient Weather or Froggit weather stations.
Development is made with a Froggit HP1000SE Pro (HP2553 clone), with the following sensors:

* WH25 (indoor sensor)
* WH80 (ultrasonic combined sensor array)
* WH40 (raingauge sensor)

Request for new sensors should be sent with an HTTP capture of your ecowitt payload. You can easily make one by running 
`nc -l -p 8000` (or `nc -l 8000` on macOS).

## Configuration

This project uses [Rocket](https://rocket.rs/) as HTTP server and for now relies on 
[its default settings](https://rocket.rs/v0.5-rc/guide/configuration/#default-provider). You can customize them with 
either a Rocket.toml file or `ROCKET_` environment variables. At the very least, you will have to set ROCKET_ADDRESS to 
listen on IP addresses other than 127.0.0.1. The default port is 8000.

## Usage

Ecowitt data should be sent to the default `/data/report/` URL path. Metrics can be retrieved at `/metrics`.

Do note that in the event of sensor failure, like if its battery is exhausted, the station will at first keep sending 
its last received metrics. After some time, it will declare the sensor dead and stop sending those metrics. Only at 
this point will metrics be culled from the exporter. No attempt will be made by this program to detect the situation 
beforehand.