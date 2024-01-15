use bme680::{
    Bme680, FieldDataCondition, I2CAddress, IIRFilterSize, OversamplingSetting, PowerMode,
    SettingsBuilder,
};
use influx_db_client::{points, Client, Point, Points, Precision, Value};
use linux_embedded_hal::*;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

const INFLUX_ADDRESS: &str = "http://127.0.0.1:8086";
const INFLUX_USER: &str = "user";
const INFLUX_PASSWORD: &str = "pass";
const INFLUX_DATABASE: &str = "influxdb";

#[tokio::main]

async fn main() -> Result<(), ()> {
    // Init device
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
   //Annabel Edit below
//    println!("{}", i2c);
    let mut delayer = Delay {};
    let mut dev = Bme680::init(i2c, &mut delayer, I2CAddress::Secondary)
        .map_err(|e| eprintln!("Init failed: {:?}", e))?;

    let settings = SettingsBuilder::new()
        .with_humidity_oversampling(OversamplingSetting::OS2x)
        .with_pressure_oversampling(OversamplingSetting::OS4x)
        .with_temperature_oversampling(OversamplingSetting::OS8x)
        .with_temperature_filter(IIRFilterSize::Size3)
        .with_gas_measurement(Duration::from_millis(1500), 320, 25)
        .with_run_gas(true)
        .build();
    dev.set_sensor_settings(&mut delayer, settings)
        .map_err(|e| eprintln!("Setting sensor settings failed: {:?}", e))?;

    let client = Client::new(Url::parse(INFLUX_ADDRESS).unwrap(), INFLUX_DATABASE)
        .set_authentication(INFLUX_USER, INFLUX_PASSWORD);

    loop {
        dev.set_sensor_mode(&mut delayer, PowerMode::ForcedMode)
            .map_err(|e| eprintln!("Setting sensor mode failed: {:?}", e))?;
        let (data, state) = dev
            .get_sensor_data(&mut delayer)
            .map_err(|e| eprintln!("Retrieving sensor data failed: {:?}", e))?;

        println!("State {:?}", state);
        println!("Temperature {}°C", data.temperature_celsius());
        println!("Pressure {}hPa", data.pressure_hpa());
        println!("Humidity {}%", data.humidity_percent());
        println!("Gas Resistence {}Ω", data.gas_resistance_ohm());
	lmssen();
        if state == FieldDataCondition::NewData {
            let temperature_f = ipoint(
                "temperature",
                Value::Float(data.temperature_celsius() as f64),
            );
            let pressure_f = ipoint("pressure", Value::Float(data.pressure_hpa() as f64));
            let humidity_f = ipoint("humidity", Value::Float(data.humidity_percent() as f64));
            let gas_f = ipoint(
                "gasresistence",
                Value::Float(data.gas_resistance_ohm() as f64),
            );

            let points = points!(temperature_f, pressure_f, humidity_f, gas_f);

            if let Err(e) = client
                .write_points(points, Some(Precision::Seconds), None)
                .await
            {
                eprintln!("Error: {:?}", e);
	    }            
        //lmssen();

        }
        sleep(Duration::from_secs(5)).await; 
//	lmssen();
    }

//lmssen();
}

/// Sends a measured value to the influx database
fn ipoint(type_name: &str, value: Value) -> Point {
    let point = Point::new("sensor")
        .add_field("value", value)
        .add_tag("id", Value::String("VMAC".to_string()))
        .add_tag("name", Value::String("bme680".to_string()))
        .add_tag("type", Value::String(type_name.to_string()));

    point
}

fn lmssen() {

   //use linux_embedded_hal::{Delay, I2cdev};
   use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

   let dev = I2cdev::new("/dev/i2c-1").unwrap();
   let mut sensor = Lsm303agr::new_with_i2c(dev);

   sensor.init().unwrap();
   sensor.set_accel_mode_and_odr(&mut Delay, AccelMode::Normal, AccelOutputDataRate::Hz10).unwrap();

   loop {
       if sensor.accel_status().unwrap().xyz_new_data() {
           let data = sensor.acceleration().unwrap();
           println!("Acceleration: x {} y {} z {}", data.x_mg(), data.y_mg(), data.z_mg());
      	   break; 
	}
   }
}

