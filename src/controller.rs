use std::{thread::sleep, time::Duration};

use anyhow::{anyhow, Ok, Result};
use embedded_hal::digital::v2::OutputPin;
use max7219::{connectors::PinConnector, MAX7219};

use crate::letters::letter_cache;

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
            if self.connection.clear_display(i).is_err() {
                eprintln!("error in clear method {}:{}", file!(), line!());
            }
        }
    }

    pub fn power_on(&mut self) {
        if self.connection.power_on().is_err() {
            eprintln!("error in power on method {}:{}", file!(), line!());
        };
    }

    pub fn power_off(&mut self) {
        if self.connection.power_off().is_err() {
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
        let mut letter_vec = letter_cache::cache_keys();
        letter_vec.sort();
        for letters in letter_vec
            .iter()
            .zip(letter_vec.iter().skip(1).chain(letter_vec.iter().take(1)))
            .zip(letter_vec.iter().skip(2).chain(letter_vec.iter().take(2)))
            .zip(letter_vec.iter().skip(3).chain(letter_vec.iter().take(3)))
            .map(|(((letter_0, letter_1), letter_2), letter_3)| {
                vec![letter_0, letter_1, letter_2, letter_3]
            })
        {
            for (i, letter) in letters.iter().enumerate() {
                if let Err(e) = self
                    .connection
                    .write_raw(i, letter_cache::get_letter(letter).unwrap().get_u8_grid())
                {
                    eprintln!("error in getting u8 grid of {}\n error: {:?}", letter, e);
                }
            }
            sleep(Duration::from_millis(500));
        }
    }

    pub fn update(&mut self) {
        todo!()
    }
}
