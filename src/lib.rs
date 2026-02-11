#![no_std]

use esp_hal::{DriverMode, delay::{self, Delay}, i2c::{self, master::{Config, I2c}}, time::Rate};

use consts::*;
#[allow(dead_code)]
pub mod consts {
    // Accesible from all user banks
    pub const REG_BANK_SEL: u8 = 0x76;
    // User Bank 0
    pub const UB0_REG_DEVICE_CONFIG: u8 = 0x11;
    // break
    pub const UB0_REG_DRIVE_CONFIG: u8 = 0x13;
    pub const UB0_REG_INT_CONFIG: u8   = 0x14;
    // break
    pub const UB0_REG_FIFO_CONFIG: u8 = 0x16;
    // break
    pub const UB0_REG_TEMP_DATA1: u8    = 0x1D;
    pub const UB0_REG_TEMP_DATA0: u8    = 0x1E;
    pub const UB0_REG_ACCEL_DATA_X1: u8 = 0x1F;
    pub const UB0_REG_ACCEL_DATA_X0: u8 = 0x20;
    pub const UB0_REG_ACCEL_DATA_Y1: u8 = 0x21;
    pub const UB0_REG_ACCEL_DATA_Y0: u8 = 0x22;
    pub const UB0_REG_ACCEL_DATA_Z1: u8 = 0x23;
    pub const UB0_REG_ACCEL_DATA_Z0: u8 = 0x24;
    pub const UB0_REG_GYRO_DATA_X1: u8  = 0x25;
    pub const UB0_REG_GYRO_DATA_X0: u8  = 0x26;
    pub const UB0_REG_GYRO_DATA_Y1: u8  = 0x27;
    pub const UB0_REG_GYRO_DATA_Y0: u8  = 0x28;
    pub const UB0_REG_GYRO_DATA_Z1: u8  = 0x29;
    pub const UB0_REG_GYRO_DATA_Z0: u8  = 0x2A;
    pub const UB0_REG_TMST_FSYNCH: u8   = 0x2B;
    pub const UB0_REG_TMST_FSYNCL: u8   = 0x2C;
    pub const UB0_REG_INT_STATUS: u8    = 0x2D;
    pub const UB0_REG_FIFO_COUNTH: u8   = 0x2E;
    pub const UB0_REG_FIFO_COUNTL: u8   = 0x2F;
    pub const UB0_REG_FIFO_DATA: u8     = 0x30;
    pub const UB0_REG_APEX_DATA0: u8    = 0x31;
    pub const UB0_REG_APEX_DATA1: u8    = 0x32;
    pub const UB0_REG_APEX_DATA2: u8    = 0x33;
    pub const UB0_REG_APEX_DATA3: u8    = 0x34;
    pub const UB0_REG_APEX_DATA4: u8    = 0x35;
    pub const UB0_REG_APEX_DATA5: u8    = 0x36;
    pub const UB0_REG_INT_STATUS2: u8   = 0x37;
    pub const UB0_REG_INT_STATUS3: u8   = 0x38;
    // break
    pub const UB0_REG_SIGNAL_PATH_RESET: u8  = 0x4B;
    pub const UB0_REG_INTF_CONFIG0: u8       = 0x4C;
    pub const UB0_REG_INTF_CONFIG1: u8       = 0x4D;
    pub const UB0_REG_PWR_MGMT0: u8          = 0x4E;
    pub const UB0_REG_GYRO_CONFIG0: u8       = 0x4F;
    pub const UB0_REG_ACCEL_CONFIG0: u8      = 0x50;
    pub const UB0_REG_GYRO_CONFIG1: u8       = 0x51;
    pub const UB0_REG_GYRO_ACCEL_CONFIG0: u8 = 0x52;
    pub const UB0_REG_ACCEFL_CONFIG1: u8     = 0x53;
    pub const UB0_REG_TMST_CONFIG: u8        = 0x54;
    // break
    pub const UB0_REG_APEX_CONFIG0: u8 = 0x56;
    pub const UB0_REG_SMD_CONFIG: u8   = 0x57;
    // break
    pub const UB0_REG_FIFO_CONFIG1: u8 = 0x5F;
    pub const UB0_REG_FIFO_CONFIG2: u8 = 0x60;
    pub const UB0_REG_FIFO_CONFIG3: u8 = 0x61;
    pub const UB0_REG_FSYNC_CONFIG: u8 = 0x62;
    pub const UB0_REG_INT_CONFIG0: u8  = 0x63;
    pub const UB0_REG_INT_CONFIG1: u8  = 0x64;
    pub const UB0_REG_INT_SOURCE0: u8  = 0x65;
    pub const UB0_REG_INT_SOURCE1: u8  = 0x66;
    // break
    pub const UB0_REG_INT_SOURCE3: u8 = 0x68;
    pub const UB0_REG_INT_SOURCE4: u8 = 0x69;
    // break
    pub const UB0_REG_FIFO_LOST_PKT0: u8 = 0x6C;
    pub const UB0_REG_FIFO_LOST_PKT1: u8 = 0x6D;
    // break
    pub const UB0_REG_SELF_TEST_CONFIG: u8 = 0x70;
    // break
    pub const UB0_REG_WHO_AM_I: u8 = 0x75;
    
    // User Bank 1
    pub const UB1_REG_SENSOR_CONFIG0: u8 = 0x03;
    // break
    pub const UB1_REG_GYRO_CONFIG_STATIC2: u8  = 0x0B;
    pub const UB1_REG_GYRO_CONFIG_STATIC3: u8  = 0x0C;
    pub const UB1_REG_GYRO_CONFIG_STATIC4: u8  = 0x0D;
    pub const UB1_REG_GYRO_CONFIG_STATIC5: u8  = 0x0E;
    pub const UB1_REG_GYRO_CONFIG_STATIC6: u8  = 0x0F;
    pub const UB1_REG_GYRO_CONFIG_STATIC7: u8  = 0x10;
    pub const UB1_REG_GYRO_CONFIG_STATIC8: u8  = 0x11;
    pub const UB1_REG_GYRO_CONFIG_STATIC9: u8  = 0x12;
    pub const UB1_REG_GYRO_CONFIG_STATIC10: u8 = 0x13;
    // break
    pub const UB1_REG_XG_ST_DATA: u8 = 0x5F;
    pub const UB1_REG_YG_ST_DATA: u8 = 0x60;
    pub const UB1_REG_ZG_ST_DATA: u8 = 0x61;
    pub const UB1_REG_TMSTVAL0: u8   = 0x62;
    pub const UB1_REG_TMSTVAL1: u8   = 0x63;
    pub const UB1_REG_TMSTVAL2: u8   = 0x64;
    // break
    pub const UB1_REG_INTF_CONFIG4: u8 = 0x7A;
    pub const UB1_REG_INTF_CONFIG5: u8 = 0x7B;
    pub const UB1_REG_INTF_CONFIG6: u8 = 0x7C;
    
    // User Bank 2
    pub const UB2_REG_ACCEL_CONFIG_STATIC2: u8 = 0x03;
    pub const UB2_REG_ACCEL_CONFIG_STATIC3: u8 = 0x04;
    pub const UB2_REG_ACCEL_CONFIG_STATIC4: u8 = 0x05;
    // break
    pub const UB2_REG_XA_ST_DATA: u8 = 0x3B;
    pub const UB2_REG_YA_ST_DATA: u8 = 0x3C;
    pub const UB2_REG_ZA_ST_DATA: u8 = 0x3D;
    
    // User Bank 4
    pub const UB4_REG_APEX_CONFIG1: u8 = 0x40;
    pub const UB4_REG_APEX_CONFIG2: u8 = 0x41;
    pub const UB4_REG_APEX_CONFIG3: u8 = 0x42;
    pub const UB4_REG_APEX_CONFIG4: u8 = 0x43;
    pub const UB4_REG_APEX_CONFIG5: u8 = 0x44;
    pub const UB4_REG_APEX_CONFIG6: u8 = 0x45;
    pub const UB4_REG_APEX_CONFIG7: u8 = 0x46;
    pub const UB4_REG_APEX_CONFIG8: u8 = 0x47;
    pub const UB4_REG_APEX_CONFIG9: u8 = 0x48;
    // break
    pub const UB4_REG_ACCEL_WOM_X_THR: u8 = 0x4A;
    pub const UB4_REG_ACCEL_WOM_Y_THR: u8 = 0x4B;
    pub const UB4_REG_ACCEL_WOM_Z_THR: u8 = 0x4C;
    pub const UB4_REG_INT_SOURCE6: u8     = 0x4D;
    pub const UB4_REG_INT_SOURCE7: u8     = 0x4E;
    pub const UB4_REG_INT_SOURCE8: u8     = 0x4F;
    pub const UB4_REG_INT_SOURCE9: u8     = 0x50;
    pub const UB4_REG_INT_SOURCE10: u8    = 0x51;
    // break
    pub const UB4_REG_OFFSET_USER0: u8 = 0x77;
    pub const UB4_REG_OFFSET_USER1: u8 = 0x78;
    pub const UB4_REG_OFFSET_USER2: u8 = 0x79;
    pub const UB4_REG_OFFSET_USER3: u8 = 0x7A;
    pub const UB4_REG_OFFSET_USER4: u8 = 0x7B;
    pub const UB4_REG_OFFSET_USER5: u8 = 0x7C;
    pub const UB4_REG_OFFSET_USER6: u8 = 0x7D;
    pub const UB4_REG_OFFSET_USER7: u8 = 0x7E;
    pub const UB4_REG_OFFSET_USER8: u8 = 0x7F;
}

#[derive(Clone, Copy)]
pub enum GyroFS {
    Dps2000   = 0x00,
    Dps1000   = 0x01,
    Dps500    = 0x02,
    Dps250    = 0x03,
    Dps125    = 0x04,
    Dps62_5   = 0x05,
    Dps31_25  = 0x06,
    Dps15_625 = 0x07
}

#[derive(Clone, Copy)]
pub enum AccelFS {
	Gpm16 = 0x00,
	Gpm8  = 0x01,
	Gpm4  = 0x02,
	Gpm2  = 0x03
}

#[derive(Clone, Copy)]
pub enum ODR {
    Odr32k    = 0x01,  // LN mode only
    Odr16k    = 0x02,  // LN mode only
    Odr8k     = 0x03,  // LN mode only
    Odr4k     = 0x04,  // LN mode only
    Odr2k     = 0x05,  // LN mode only
    Odr1k     = 0x06,  // LN mode only
    Odr200    = 0x07,
    Odr100    = 0x08,
    Odr50     = 0x09,
    Odr25     = 0x0A,
    Odr12_5   = 0x0B,
    Odr6a25   = 0x0C,  // LP mode only (accel only)
    Odr3a125  = 0x0D,  // LP mode only (accel only)
    Odr1a5625 = 0x0E,  // LP mode only (accel only)
    Odr500    = 0x0F,
}

#[derive(Clone, Copy)]
pub enum GyroNFBWsel {
    NfBW1449Hz = 0x00,
    NfBW680z   = 0x01,
    NfBW329Hz  = 0x02,
    NfBW162Hz  = 0x03,
    NfBW80Hz   = 0x04,
    NfBW40Hz   = 0x05,
    NfBW20Hz   = 0x06,
    NfBW10Hz   = 0x07,
}

#[derive(Clone, Copy)]
pub enum UIFiltOrd {
    FirstOrder  = 0x00,
    SecondOrder = 0x01,
    ThirdOrder  = 0x02,
}

pub struct Icm42688<'a, 'b: 'a, D: DriverMode> {
    address: u8,
    i2c: &'a mut I2c<'b, D>,

    // todo spi but im not doing it

    t: f32,
    acc: [f32; 3],
    gyr: [f32; 3],

    raw_t: i16,
    raw_acc: [i16; 3],
    raw_gyr: [i16; 3],

    raw_acc_bias: [i32; 3],
    raw_gyr_bias: [i32; 3],
    // acc_offset: [i16; 3],
    // gyr_offset: [i16; 3],

    acc_scale: f32,
    gyro_scale: f32,

    accel_fs: AccelFS,
    gyro_fs: GyroFS,

    // acc_bd: [f32; 3],
    pub acc_b: [f32; 3],
    acc_s: [f32; 3],
    // acc_max: [f32; 3],
    // acc_min: [f32; 3],

    gyr_bd: [f32; 3],
    pub gyr_b: [f32; 3],

    bank: u8,
}

impl<'a, 'b: 'a, D: DriverMode> Icm42688<'a, 'b, D> {
    const I2C_CLK: u32 = 400_000;
    const WHOAMI: u8 = 0x47;
    const NUM_CALIB_SAMPLES: i32 = 1000;
    const TEMP_DATA_REG_SCALE: f32 = 123.48;
    const TEMP_OFFSET: f32 = 25.0;

    // const FIFO_EN: u8 = 0x5F;
    // const FIFO_TEMP_EN: u8 = 0x04;
    // const FIFO_GYRO: u8 = 0x02;
    // const FIFO_ACCEL: u8 = 0x01;

    const GYRO_NF_ENABLE: u8 = 0x00;
    const GYRO_NF_DISABLE: u8 = 0x01;
    const GYRO_AAF_ENABLE: u8 = 0x00;
    const GYRO_AAF_DISABLE: u8 = 0x02;

    const ACCEL_AAF_ENABLE: u8 = 0x00;
    const ACCEL_AAF_DISABLE: u8 = 0x01;

    pub fn write_register(&mut self, sub_addr: u8, data: u8) -> Result<(), Error> {
        self.i2c.write(self.address, &[sub_addr, data])?;
        Ok(())
    }
    pub fn read_registers(&mut self, sub_addr: u8, dest: &mut [u8]) -> Result<(), Error> {
        self.i2c.write_read(self.address, &[sub_addr], dest)?;
        Ok(())
    }
    pub fn read_register_byte(&mut self, sub_addr: u8) -> Result<u8, Error> {
        let mut buf = [0];
        self.i2c.write_read(self.address, &[sub_addr], &mut buf)?;
        Ok(buf[0])
    }
    pub fn set_bank(&mut self, bank: u8) -> Result<(), Error> {
        if self.bank == bank { return Ok(()) }
        self.bank = bank;
        self.write_register(REG_BANK_SEL, bank)
    }

    fn reset(&mut self) -> Result<(), Error> {
        self.set_bank(0)?;
        self.write_register(UB0_REG_DEVICE_CONFIG, 0x01)?;
        let delay = delay::Delay::new();
        delay.delay_millis(1);
        Ok(())
    }

    fn who_am_i(&mut self) -> Result<u8, Error> {
        self.set_bank(0)?;
        let byte = self.read_register_byte(UB0_REG_WHO_AM_I)?;
        Ok(byte)
    }

    // public

    pub fn new(bus: &'a mut I2c<'b, D>, addr: u8) -> Self {
        Self {
            address: addr,
            i2c: bus,
            t: 0.,
            acc: [0.; 3],
            gyr: [0.; 3],
            raw_t: 0,
            raw_acc: [0; 3],
            raw_gyr: [0; 3],
            raw_acc_bias: [0; 3],
            raw_gyr_bias: [0; 3],
            acc_scale: 0.,
            gyro_scale: 0.,
            accel_fs: AccelFS::Gpm16,
            gyro_fs: GyroFS::Dps2000,
            acc_b: [0.; 3],
            acc_s: [1.; 3],
            gyr_bd: [0.; 3],
            gyr_b: [0.; 3],
            bank: 0,
        }
    }

    pub fn begin(&mut self) -> Result<(), Error> {
        self.i2c.apply_config(&Config::default().with_frequency(Rate::from_hz(Self::I2C_CLK)))?;
        self.reset()?;

        if self.who_am_i()? != Self::WHOAMI {
            return Err(Error::WhoAmIError)
        }

        self.write_register(UB0_REG_PWR_MGMT0, 0x0F)?;

        self.set_accel_fs(AccelFS::Gpm16)?;
        self.set_gyro_fs(GyroFS::Dps2000)?;
        self.set_filters(false, false)?;
        self.calibrate_gyro()?;

        Ok(())
    }

    // so that the driver can be dropped and the ic doesnt need to be reset the next time it is create
    pub fn detect_config(&mut self) -> Result<(), Error> {
        self.accel_fs = self.get_accel_fs()?;
        self.gyro_fs = self.get_gyro_fs()?;
        self.acc_scale = (1 << (4 - self.accel_fs as u8)) as f32 / 32768.;
        self.gyro_scale = (2000. / (1 << (self.gyro_fs as u8)) as f32) / 32768.;
        self.calibrate_gyro()?;

        Ok(())
    }

    pub fn calibrate_gyro(&mut self) -> Result<(), Error> {
        let current_fs = self.gyro_fs;
        self.set_gyro_fs(GyroFS::Dps250)?;

        self.gyr_bd = [0.; 3];
        let delay = Delay::new();
        for _ in 0..Self::NUM_CALIB_SAMPLES {
            self.get_agt()?;
            self.gyr_bd[0] += (self.gyr[0] + self.gyr_b[0]) / Self::NUM_CALIB_SAMPLES as f32;
            self.gyr_bd[1] += (self.gyr[1] + self.gyr_b[1]) / Self::NUM_CALIB_SAMPLES as f32;
            self.gyr_bd[2] += (self.gyr[2] + self.gyr_b[2]) / Self::NUM_CALIB_SAMPLES as f32;
            delay.delay_millis(1);
        }
        self.gyr_b = self.gyr_bd;

        self.set_gyro_fs(current_fs)?;
        
        Ok(())
    }

    pub fn set_accel_fs(&mut self, fssel: AccelFS) -> Result<(), Error> {
        self.set_bank(0)?;
        let mut reg = self.read_register_byte(UB0_REG_ACCEL_CONFIG0)?;
        reg = ((fssel as u8) << 5) | (reg & 0x1F);
        self.write_register(UB0_REG_ACCEL_CONFIG0, reg)?;
        self.acc_scale = (1 << (4 - fssel as u8)) as f32 / 32768.;
        self.accel_fs = fssel;
        Ok(())
    }
    pub fn get_accel_fs(&mut self) -> Result<AccelFS, Error> {
        self.set_bank(0)?;
        let reg = self.read_register_byte(UB0_REG_ACCEL_CONFIG0)?;
        let ret = match (reg & 0xE0) >> 5 {
            0x00 => AccelFS::Gpm16,
            0x01 => AccelFS::Gpm8,
            0x02 => AccelFS::Gpm4,
            0x03 => AccelFS::Gpm2,
            _ => panic!("hopefully this doesnt happen..?")
        };
        Ok(ret)
    }
    pub fn set_gyro_fs(&mut self, fssel: GyroFS) -> Result<(), Error> {
        self.set_bank(0)?;
        let mut reg = self.read_register_byte(UB0_REG_GYRO_CONFIG0)?;
        reg = ((fssel as u8) << 5) | (reg & 0x1F);
        self.write_register(UB0_REG_GYRO_CONFIG0, reg)?;
        self.gyro_scale = (2000. / (1 << (fssel as u8)) as f32) / 32768.;
        self.gyro_fs = fssel;
        Ok(())
    }
    pub fn get_gyro_fs(&mut self) -> Result<GyroFS, Error> {
        self.set_bank(0)?;
        let reg = self.read_register_byte(UB0_REG_GYRO_CONFIG0)?;
        let ret = match (reg & 0xE0) >> 5 {
            0x00 => GyroFS::Dps2000,
            0x01 => GyroFS::Dps1000,
            0x02 => GyroFS::Dps500,
            0x03 => GyroFS::Dps250,
            0x04 => GyroFS::Dps125,
            0x05 => GyroFS::Dps62_5,
            0x06 => GyroFS::Dps31_25,
            0x07 => GyroFS::Dps15_625,
            _ => panic!("hopefully this doesnt happen..?")
        };
        Ok(ret)
    }
    pub fn set_gyro_odr(&mut self, odr: ODR) -> Result<(), Error> {
        self.set_bank(0)?;
        let mut reg = self.read_register_byte(UB0_REG_GYRO_CONFIG0)?;
        reg = odr as u8 | (reg & 0xF0);
        self.write_register(UB0_REG_GYRO_CONFIG0, reg)?;
        Ok(())
    }
    /*pub fn get_gyro_odr(&mut self) -> Result<i32, Error> {

    } ????? */ 
    pub fn set_accel_odr(&mut self, odr: ODR) -> Result<(), Error> {
        self.set_bank(0)?;
        let mut reg = self.read_register_byte(UB0_REG_ACCEL_CONFIG0)?;
        reg = odr as u8 | (reg & 0xF0);
        self.write_register(UB0_REG_ACCEL_CONFIG0, reg)?;
        Ok(())
    }
    /*pub fn get_accel_odr(&mut self) -> Result<i32, Error> {

    } ????????? */ 

    pub fn set_filters(&mut self, gyro_filters: bool, acc_filters: bool) -> Result<(), Error> {
        self.set_bank(1)?;

        if gyro_filters {
            self.write_register(UB1_REG_GYRO_CONFIG_STATIC2, Self::GYRO_NF_ENABLE | Self::GYRO_AAF_ENABLE)?;
        } else {
            self.write_register(UB1_REG_GYRO_CONFIG_STATIC2, Self::GYRO_NF_DISABLE | Self::GYRO_AAF_DISABLE)?;
        }
        
        self.set_bank(2)?;

        if acc_filters {
            self.write_register(UB2_REG_ACCEL_CONFIG_STATIC2, Self::ACCEL_AAF_ENABLE)?;
        } else {
            self.write_register(UB2_REG_ACCEL_CONFIG_STATIC2, Self::ACCEL_AAF_DISABLE)?;
        }

        self.set_bank(0)?;
        Ok(())
    }
    
    pub fn enable_data_ready_interrupt(&mut self) -> Result<(), Error> {
        self.set_bank(0)?;
        self.write_register(UB0_REG_INT_CONFIG, 0x18 | 0x03)?;
        let mut reg = self.read_register_byte(UB0_REG_INTF_CONFIG1)?;
        reg &= !0x10;
        self.write_register(UB0_REG_INT_CONFIG, reg)?;

        self.write_register(UB0_REG_INT_SOURCE0, 0x18)
    }

    pub fn disable_data_ready_interrupt(&mut self) -> Result<(), Error> {
        self.set_bank(0)?;
        self.write_register(UB0_REG_INT_CONFIG1, 0x18 | 0x03)?;
        let mut reg = self.read_register_byte(UB0_REG_INTF_CONFIG1)?;
        reg &= !0x10;
        self.write_register(UB0_REG_INT_CONFIG1, reg)?;

        self.write_register(UB0_REG_INT_SOURCE0, 0x18)
    }

    pub fn get_agt(&mut self) -> Result<(), Error> {
        self.set_bank(0)?;
        self.get_raw_agt()?;

        self.t = (self.raw_t as f32 / Self::TEMP_DATA_REG_SCALE) + Self::TEMP_OFFSET;

        self.acc[0] = ((self.raw_acc[0] as f32 * self.acc_scale) - self.acc_b[0]) * self.acc_s[0];
        self.acc[1] = ((self.raw_acc[1] as f32 * self.acc_scale) - self.acc_b[1]) * self.acc_s[1];
        self.acc[2] = ((self.raw_acc[2] as f32 * self.acc_scale) - self.acc_b[2]) * self.acc_s[2];

        self.gyr[0] = (self.raw_gyr[0] as f32 * self.gyro_scale) - self.gyr_b[0];
        self.gyr[1] = (self.raw_gyr[1] as f32 * self.gyro_scale) - self.gyr_b[1];
        self.gyr[2] = (self.raw_gyr[2] as f32 * self.gyro_scale) - self.gyr_b[2];

        Ok(())
    }
    pub fn get_raw_agt(&mut self) -> Result<(), Error> {
        self.set_bank(0)?;
        let mut buf = [0; 14];
        self.read_registers(UB0_REG_TEMP_DATA1, &mut buf)?;
        let mut raw_meas = [0;7];
        for i in 0..7 {
            raw_meas[i] = ((buf[i * 2] as i16) << 8) | buf[i * 2 + 1] as i16
        }

        self.raw_t = raw_meas[0];
        self.raw_acc[0] = raw_meas[1];
        self.raw_acc[1] = raw_meas[2];
        self.raw_acc[2] = raw_meas[3];
        self.raw_gyr[0] = raw_meas[4];
        self.raw_gyr[1] = raw_meas[5];
        self.raw_gyr[2] = raw_meas[6];

        Ok(())
    }

    pub const fn acc(&mut self) -> [f32;3] { return self.acc }
    pub const fn gyr(&mut self) -> [f32;3] { return self.gyr }

    pub const fn temp(&mut self) -> f32 { return self.t }

    pub const fn raw_acc(&mut self) -> [i16;3] { return self.raw_acc }
    pub const fn raw_gyr(&mut self) -> [i16;3] { return self.raw_gyr }

    pub const fn raw_temp(&mut self) -> i16 { return self.raw_t }

    pub const fn raw_acc_bias(&mut self) -> [i32;3] { return self.raw_acc_bias }
    pub const fn raw_gyr_bias(&mut self) -> [i32;3] { return self.raw_gyr_bias }

    pub fn set_acc_x_offset(&mut self, offset: i16) -> Result<(), Error> {
        self.set_bank(4)?;
        let reg1: u8 = (offset & 0x00FF) as u8;
        let mut reg2 = self.read_register_byte(UB4_REG_OFFSET_USER4)?;
        reg2 = (reg2 & 0x0F) | ((offset & 0x0F00) >> 4) as u8;
        self.write_register(UB4_REG_OFFSET_USER5, reg1)?;
        self.write_register(UB4_REG_OFFSET_USER4, reg2)
    }
    pub fn set_acc_y_offset(&mut self, offset: i16) -> Result<(), Error> {
        self.set_bank(4)?;
        let reg1: u8 = (offset & 0x00FF) as u8;
        let mut reg2 = self.read_register_byte(UB4_REG_OFFSET_USER7)?;
        reg2 = (reg2 & 0x0F) | ((offset & 0x0F00) >> 4) as u8;
        self.write_register(UB4_REG_OFFSET_USER6, reg1)?;
        self.write_register(UB4_REG_OFFSET_USER7, reg2)
    }
    pub fn set_acc_z_offset(&mut self, offset: i16) -> Result<(), Error> {
        self.set_bank(4)?;
        let reg1: u8 = (offset & 0x00FF) as u8;
        let mut reg2 = self.read_register_byte(UB4_REG_OFFSET_USER7)?;
        reg2 = (reg2 & 0x0F) | ((offset & 0x0F00) >> 4) as u8;
        self.write_register(UB4_REG_OFFSET_USER8, reg1)?;
        self.write_register(UB4_REG_OFFSET_USER7, reg2)
    }

    pub fn set_gyr_x_offset(&mut self, offset: i16) -> Result<(), Error> {
        self.set_bank(4)?;
        let reg1: u8 = (offset & 0x00FF) as u8;
        let mut reg2 = self.read_register_byte(UB4_REG_OFFSET_USER1)?;
        reg2 = (reg2 & 0x0F) | ((offset & 0x0F00) >> 4) as u8;
        self.write_register(UB4_REG_OFFSET_USER0, reg1)?;
        self.write_register(UB4_REG_OFFSET_USER1, reg2)
    }
    pub fn set_gyr_y_offset(&mut self, offset: i16) -> Result<(), Error> {
        self.set_bank(4)?;
        let reg1: u8 = (offset & 0x00FF) as u8;
        let mut reg2 = self.read_register_byte(UB4_REG_OFFSET_USER1)?;
        reg2 = (reg2 & 0x0F) | ((offset & 0x0F00) >> 4) as u8;
        self.write_register(UB4_REG_OFFSET_USER2, reg1)?;
        self.write_register(UB4_REG_OFFSET_USER1, reg2)
    }
    pub fn set_gyr_z_offset(&mut self, offset: i16) -> Result<(), Error> {
        self.set_bank(4)?;
        let reg1: u8 = (offset & 0x00FF) as u8;
        let mut reg2 = self.read_register_byte(UB4_REG_OFFSET_USER4)?;
        reg2 = (reg2 & 0x0F) | ((offset & 0x0F00) >> 4) as u8;
        self.write_register(UB4_REG_OFFSET_USER3, reg1)?;
        self.write_register(UB4_REG_OFFSET_USER4, reg2)
    }

    pub fn get_accel_res(&mut self) -> Result<f32, Error>{
        let current_accfs = self.get_accel_fs()?;
        let acc_res = match current_accfs {
            // todo is this correct? the cpp library does this but surely its a mistake
            AccelFS::Gpm16 | AccelFS::Gpm2 => 16. / 32768.,
            AccelFS::Gpm8 => 8. / 32768.,
            AccelFS::Gpm4 => 4. / 32768.,
        };
        Ok(acc_res)
    }
    pub fn get_gyro_res(&mut self) -> Result<f32, Error>{
        let current_gyrfs = self.get_gyro_fs()?;
        let gyr_res = match current_gyrfs {
            GyroFS::Dps2000 => 2000. / 32768.,
            GyroFS::Dps1000 => 1000. / 32768.,
            GyroFS::Dps500 => 500. / 32768.,
            GyroFS::Dps250 => 250. / 32768.,
            GyroFS::Dps125 => 125. / 32768.,
            GyroFS::Dps62_5 => 62.6 / 32768.,
            GyroFS::Dps31_25 => 31.25 / 32768.,
            GyroFS::Dps15_625 => 15.625 / 32768.,
        };
        Ok(gyr_res)
    }

    // todos: notch filter
    // aaf filter
    // ui filter block
    // self test
    // fifo
    // spi
    // compute & set all offset biases

    // calibrate acceleration - the function confused me so i didnt wanna do it

    // ^^^ a lot here isnt done because i dont need it and im kinda tired of translating all this code
    // also i may be missing stuff im just going off of the cpp library. someone could check the datasheet but that wont be me

    /*
    and whatever this is from the cpp library
    //Low priority
    Read INT_STATUS          <-- get info if data are available

    ApexStatus   => INT_STATUS2 and INT_STATUS3
    8.1 APEX ODR SUPPORT
    8.2 DMP POWER SAVE MODE
    8.3 PEDOMETER PROGRAMMING
    8.4 TILT DETECTION PROGRAMMING
    8.5 RAISE TO WAKE/SLEEP PROGRAMMING
    8.6 TAP DETECTION PROGRAMMING
    8.7 WAKE ON MOTION PROGRAMMING
    8.8 SIGNIFICANT MOTION DETECTION PROGRAMMING  p47
     */
}

#[derive(Debug)]
pub enum Error {
    I2cError(i2c::master::Error),
    ConfigError(i2c::master::ConfigError),
    WhoAmIError,
}
impl From<i2c::master::Error> for Error {
    fn from(value: i2c::master::Error) -> Self {
        Error::I2cError(value)
    }
}
impl From<i2c::master::ConfigError> for Error {
    fn from(value: i2c::master::ConfigError) -> Self {
        Error::ConfigError(value)
    }
}