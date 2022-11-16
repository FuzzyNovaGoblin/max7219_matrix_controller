use self::ctrl_state::CtrlState;
use crate::symbols::symbol_cache;
use anyhow::{anyhow, Ok, Result};
use embedded_hal::digital::v2::OutputPin;
use std::{thread::sleep, time::Duration};

pub mod ctrl_state;
mod ctrl_state_atributes;

#[derive(Clone, Copy)]
pub enum Command {
    Noop = 0x00,
    Digit0 = 0x01,
    Digit1 = 0x02,
    Digit2 = 0x03,
    Digit3 = 0x04,
    Digit4 = 0x05,
    Digit5 = 0x06,
    Digit6 = 0x07,
    Digit7 = 0x08,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Power = 0x0C,
    DisplayTest = 0x0F,
}

#[derive(Copy, Clone)]
pub enum DecodeMode {
    NoDecode = 0x00,
    CodeBDigit0 = 0x01,
    CodeBDigits3_0 = 0x0F,
    CodeBDigits7_0 = 0xFF,
}

pub struct MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    module_count: u8,
    state: CtrlState,
    data: DATA,
    cs: CS,
    clk: CLK,
    buffer: Vec<u8>,
}

const MAX_DIGITS: usize = 8;

/// controller functions
impl<DATA, CS, CLK> MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
}

// initialization functions
impl<DATA, CS, CLK> MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    pub fn new(module_count: u8, data: DATA, cs: CS, clk: CLK) -> Result<Self> {
        let mut ret_val = MaxController {
            module_count,
            state: ctrl_state::CtrlState::new(module_count),
            data,
            cs,
            clk,
            buffer: vec![0; 8 * 2],
        };

        ret_val.init()?;

        Ok(ret_val)
    }

    fn init(&mut self) -> Result<()> {
        for i in 0..self.module_count {
            self.set_test_mode(i, false)?;
            self.write_raw(i, Command::ScanLimit as u8, 7)?;
            self.write_raw(i, Command::DecodeMode as u8, 0)?;
            self.clear_display(i)?; // clear all digits
        }
        self.power_off()?; // power off

        Ok(())
    }
}

/// write functions
impl<DATA, CS, CLK> MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    pub fn write(&mut self, addr: u8, raw: &[u8; MAX_DIGITS]) -> Result<()> {
        let mut digit: u8 = 0;
        for b in raw {
            self.write_raw(addr, digit % 8 + 1, *b)?;
            digit += 1;
        }
        Ok(())
    }

    pub fn write_raw(&mut self, addr: u8, header: u8, data: u8) -> Result<()> {
        println!("addr {}\theader {}\tdata {}", addr, header, data);
        let offset = addr as usize * 2;
        let max_bytes = self.module_count as usize * 2;
        self.buffer = vec![0; max_bytes];

        self.buffer[offset] = header;
        self.buffer[offset + 1] = data;

        self.cs
            .set_low()
            .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        for b in 0..max_bytes {
            let value = self.buffer[b];

            for i in 0..8 {
                if value & (1 << (7 - i)) != 0 {
                    self.data
                        .set_high()
                        .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
                } else {
                    self.data
                        .set_low()
                        .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
                }

                self.clk
                    .set_high()
                    .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
                self.clk
                    .set_low()
                    .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
            }
        }
        self.cs
            .set_high()
            .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;

        Ok(())
    }
}

/// test functions
impl<DATA, CS, CLK> MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    ///
    pub fn test_letter(&mut self) -> Result<()> {
        for (i, symbol) in ('A'..='Z').enumerate() {
            if symbol_cache::is_available_symbol(&symbol) {
                self.write(
                    i as u8 % self.module_count,
                    symbol_cache::get_symbol(&symbol).unwrap().get_u8_grid(),
                )?;
                sleep(Duration::from_millis(700));
            }
        }
        Ok(())
    }

    pub fn test_symbol(&mut self) {
        let mut symbol_vec = symbol_cache::cache_keys();
        symbol_vec.sort();
        for symbols in symbol_vec
            .iter()
            .zip(symbol_vec.iter().skip(1).chain(symbol_vec.iter().take(1)))
            .zip(symbol_vec.iter().skip(2).chain(symbol_vec.iter().take(2)))
            .zip(symbol_vec.iter().skip(3).chain(symbol_vec.iter().take(3)))
            .map(|(((symbol_0, symbol_1), symbol_2), symbol_3)| {
                vec![symbol_0, symbol_1, symbol_2, symbol_3]
            })
        {
            for (i, symbol) in symbols.iter().enumerate() {
                if let Err(e) = self.write(
                    i as u8,
                    symbol_cache::get_symbol(symbol).unwrap().get_u8_grid(),
                ) {
                    eprintln!("error in getting u8 grid of {}\n error: {:?}", symbol, e);
                }
            }
            sleep(Duration::from_millis(500));
        }
    }

    /// set each segment to test mode 1 by 1
    pub fn run_test(&mut self, sleep_time: Duration) -> anyhow::Result<()> {
        for mod_addr in 0..self.module_count {
            self.set_test_mode(mod_addr, true)?;

            sleep(sleep_time);

            self.set_test_mode(mod_addr, false)?;
        }
        Ok(())
    }
}

/// max7219 commands
impl<DATA, CS, CLK> MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    pub fn clear(&mut self) -> Result<()> {
        for addr in 0..self.module_count {
            for i in 1..9 {
                self.write_raw(addr, i, 0)?;
            }
        }

        Ok(())
    }

    pub fn clear_display(&mut self, addr: u8) -> Result<()> {
        for i in 1..9 {
            self.write_raw(addr, i, 0)?;
        }

        Ok(())
    }
    pub fn set_intensity(&mut self, addr: u8, intensity: u8) -> Result<()> {
        self.write_raw(addr, Command::Intensity as u8, intensity)
    }

    pub fn power_on(&mut self) -> Result<()> {
        for i in 0..self.module_count {
            self.write_raw(i, Command::Power as u8, 1)?;
        }
        Ok(())
    }

    pub fn power_off(&mut self) -> Result<()> {
        for i in 0..self.module_count {
            self.write_raw(i, Command::Power as u8, 0)?;
        }
        Ok(())
    }

    pub fn set_test_mode(&mut self, addr: u8, is_on: bool) -> Result<()> {
        if is_on {
            self.write_raw(addr, Command::DisplayTest as u8, 0x01)
        } else {
            self.write_raw(addr, Command::DisplayTest as u8, 0x00)
        }
    }

    pub fn update(&mut self) {
        todo!()
    }
}
