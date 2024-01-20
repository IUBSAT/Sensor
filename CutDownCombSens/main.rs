use bme680::{
    Bme680, I2CAddress, IIRFilterSize, OversamplingSetting, PowerMode,
    SettingsBuilder,
};

use linux_embedded_hal::*;
use std::time::Duration;
use tokio::time::sleep;







#[tokio::main]

async fn main() -> Result<(), ()> {
    // Init device
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();


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



















            



        sleep(Duration::from_secs(5)).await; 

    }


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

