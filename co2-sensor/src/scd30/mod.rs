use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use crc_all::Crc;
use nrf52840_hal::{
    pac::TIMER0,
    twim::{Error, Instance, Twim},
    Timer,
};

pub struct SensorData {
    pub co2: f32,
    pub temperature: f32,
    pub humidity: f32,
}

const DEFAULT_ADDRESS: u8 = 0x61;
pub struct SCD30<T: Instance>(Twim<T>);

impl<T> SCD30<T>
where
    T: Instance,
{
    pub fn init(i2c2: Twim<T>) -> Self {
        SCD30(i2c2)
    }

    pub fn get_firmware_version(&mut self) -> Result<[u8; 3], Error> {
        let command: [u8; 2] = [0xd1, 0x00];
        let mut rd_buffer = [0u8; 3];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        let major = u8::from_be(rd_buffer[0]);
        let minor = u8::from_be(rd_buffer[1]);
        let crc = u8::from_be(rd_buffer[2]);

        Ok([major, minor, crc])
    }

    pub fn start_continuous_measurement(&mut self, pressure: u16) -> Result<(), Error> {
        let mut command: [u8; 5] = [0x00, 0x10, 0x00, 0x00, 0x00];
        let argument_bytes = &pressure.to_be_bytes();

        command[2] = argument_bytes[0];
        command[3] = argument_bytes[1];

        let mut crc = Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);

        crc.update(&pressure.to_be_bytes());
        command[4] = crc.finish();

        self.0.write(DEFAULT_ADDRESS, &command)?;

        Ok(())
    }

    pub fn stop_continuous_measurement(&mut self) -> Result<(), Error> {
        let command: [u8; 2] = [0x01, 0x04];

        self.0.write(DEFAULT_ADDRESS, &command)?;

        Ok(())
    }

    pub fn data_ready(&mut self, timer: &mut Timer<TIMER0>) -> Result<bool, Error> {
        let mut command: [u8; 2] = [0x02, 0x02];
        let mut rd_buffer = [0u8; 2];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        timer.delay_ms(5_u32);
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)
    }

    pub fn read_measurement(&mut self, timer: &mut Timer<TIMER0>) -> Result<SensorData, Error> {
        let command: [u8; 2] = [0x03, 0x00];
        let mut rd_buffer = [0u8; 18];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        timer.delay_ms(5_u32);
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        let data = SensorData {
            co2: f32::from_bits(u32::from_be_bytes([
                rd_buffer[0],
                rd_buffer[1],
                rd_buffer[3],
                rd_buffer[4],
            ])),
            temperature: f32::from_bits(u32::from_be_bytes([
                rd_buffer[6],
                rd_buffer[7],
                rd_buffer[9],
                rd_buffer[10],
            ])),
            humidity: f32::from_bits(u32::from_be_bytes([
                rd_buffer[12],
                rd_buffer[13],
                rd_buffer[15],
                rd_buffer[16],
            ])),
        };
        Ok(data)
    }

    // deactivates the automatic self calibration
    pub fn deactivate_self_calibration(&mut self) -> Result<(), Error> {
        let mut command: [u8; 5] = [0x53, 0x06, 0x00, 0x00, 0x00];

        let mut crc = Crc::<u8>::new(0x31, 8, 0xFF, 0x00, false);

        let asc_data = [0x00, 0x00];
        crc.update(&asc_data);
        command[4] = crc.finish();

        self.0.write(DEFAULT_ADDRESS, &command)?;

        Ok(())
    }

    // activates the automatic self calibration
    pub fn activate_self_calibration(&mut self) -> Result<(), Error> {
        let mut command: [u8; 5] = [0x53, 0x06, 0x00, 0x01, 0x00];

        let mut crc = Crc::<u8>::new(0x31, 8, 0xFF, 0x00, false);

        crc.update(&[command[2], command[3]]);
        command[4] = crc.finish();

        self.0.write(DEFAULT_ADDRESS, &command)?;

        Ok(())
    }

    pub fn get_asc_status(&mut self) -> Result<bool, Error> {
        let command: [u8; 2] = [0x53, 0x06];
        let mut rd_buffer = [0u8; 2];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)
    }

    pub fn set_altitude_compensation(&mut self, altitude: u16) -> Result<(), Error> {
        let mut command = [0x51, 0x02, 0x00, 0x00, 0x00];
        let arguement_bytes = &altitude.to_be_bytes();

        command[2] = arguement_bytes[0];
        command[3] = arguement_bytes[1];

        let mut crc = Crc::<u8>::new(0x31, 8, 0xFF, 0x00, false);

        crc.update(&altitude.to_be_bytes());
        command[4] = crc.finish();

        self.0.write(DEFAULT_ADDRESS, &command)?;

        Ok(())
    }

    pub fn get_altitude(&mut self) -> Result<u16, Error> {
        let command = [0x51, 0x02];
        let mut rd_buffer = [0u8; 2];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        let result = u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]);

        Ok(result)
    }

    pub fn soft_reset(&mut self) -> Result<(), Error> {
        let command = [0xd3, 0x04];
        
        self.0.write(DEFAULT_ADDRESS, &command)?;

        Ok(())
    }
}
