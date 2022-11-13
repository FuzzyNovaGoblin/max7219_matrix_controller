use std::{fmt::format, thread::sleep, time::Duration};

use anyhow::{anyhow, Ok, Result};
use embedded_hal::digital::v2::OutputPin;
use max7219::{connectors::PinConnector, MAX7219};

use crate::letters::{letter, letter_cache};

pub struct MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    module_count: usize,
    connection: MAX7219<PinConnector<DATA, CS, CLK>>,
}

impl<DATA, CS, CLK> MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    pub fn new(
        module_count: usize,
        data: DATA,
        cs: CS,
        sck: CLK,
    ) -> Result<MaxController<DATA, CS, CLK>> {
        Ok(MaxController {
            module_count,
            connection: MAX7219::from_pins(module_count, data, cs, sck)
                .map_err(|_| anyhow!("error ocured creating underlying max7219 controller"))?,
        })
    }

    pub fn clear(&mut self) {
        for i in 0..self.module_count {
            if let Err(_) = self.connection.clear_display(i) {
                eprintln!("error in clear method {}:{}", file!(), line!());
            }
        }
    }

    pub fn power_on(&mut self) {
        if let Err(_) = self.connection.power_on() {
            eprintln!("error in power on method {}:{}", file!(), line!());
        };
    }

    pub fn power_off(&mut self) {
        if let Err(_) = self.connection.power_off() {
            eprintln!("error in power off method {}:{}", file!(), line!());
        };
    }

    pub fn test(&mut self, sleep_time: Duration) -> anyhow::Result<()> {
        for mod_addr in 0..self.module_count {
            self.connection
                .test(mod_addr, true)
                .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;

            sleep(sleep_time);

            self.connection
                .test(mod_addr, false)
                .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        }
        Ok(())
    }

    pub fn test_letter(&mut self) {
        for letter in 'A'..='Z' {
            // for letter in letter_cache::available_letters(){
            if letter_cache::is_available_letter(&letter) {
                self.connection
                    .write_raw(2, letter_cache::get_letter(&letter).unwrap().get_u8_grid());
                sleep(Duration::from_millis(700));
            }
        }
    }

    pub fn update(&mut self) {
        todo!()
    }
}
