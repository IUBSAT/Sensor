import time
import smbus2
import bme680

# LSM303AGR registers
LSM303AGR_ADDR = 0x19  # I2C address for LSM303AGR
LSM303AGR_REG_ACCEL_X_LSB = 0x28
LSM303AGR_REG_ACCEL_X_MSB = 0x29
LSM303AGR_REG_ACCEL_Y_LSB = 0x2A
LSM303AGR_REG_ACCEL_Y_MSB = 0x2B
LSM303AGR_REG_ACCEL_Z_LSB = 0x2C
LSM303AGR_REG_ACCEL_Z_MSB = 0x2D

# BME680 setup
bme = bme680.BME680(0x77)

# LSM303AGR setup
bus = smbus2.SMBus(1)  # Use bus 1 for Raspberry Pi
bus.write_byte_data(LSM303AGR_ADDR, 0x20, 0x27)  # Enable accelerometer, set to normal mode

def read_bme680():
    temperature = bme.data.temperature
    pressure = bme.data.pressure
    humidity = bme.data.humidity
    return temperature, pressure, humidity

def read_lsm303agr():
    def read_signed_data(register):
        low_byte = bus.read_byte_data(LSM303AGR_ADDR, register)
        high_byte = bus.read_byte_data(LSM303AGR_ADDR, register + 1)
        value = (high_byte << 8) | low_byte
        return value if value < 32768 else value - 65536

    accel_x = read_signed_data(LSM303AGR_REG_ACCEL_X_LSB)
    accel_y = read_signed_data(LSM303AGR_REG_ACCEL_Y_LSB)
    accel_z = read_signed_data(LSM303AGR_REG_ACCEL_Z_LSB)
    return accel_x, accel_y, accel_z

def main():
    try:
        while True:
            temperature, pressure, humidity = read_bme680()
            accel_x, accel_y, accel_z = read_lsm303agr()

            print(f"BME680 - Temperature: {temperature:.2f}°C, Pressure: {pressure:.2f} hPa, Humidity: {humidity:.2f}%")
            print(f"LSM303AGR - Acceleration - X: {accel_x}, Y: {accel_y}, Z: {accel_z}")

            time.sleep(1)

    except KeyboardInterrupt:
        pass

if __name__ == "__main__":
    main()

