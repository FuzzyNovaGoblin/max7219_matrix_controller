use self::ctrl_state::CtrlState;
use crate::{symbols::symbol_cache, to_bits::ToBits};
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

pub struct MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    module_count: u8,
    // connection: MAX7219<PinConnector<DATA, CS, CLK>>,
    state: CtrlState,
    data: DATA,
    cs: CS,
    clk: CLK,
    buffer: [u8; 8 * 2],
}

impl<DATA, CS, CLK> MaxController<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    pub fn new(
        module_count: u8,
        data: DATA,
        cs: CS,
        clk: CLK,
    ) -> Result<MaxController<DATA, CS, CLK>> {
        println!("{}", module_count);
        Ok(MaxController {
            module_count,
            state: ctrl_state::CtrlState::new(module_count),
            data,
            cs,
            clk,
            buffer: [0; 8 * 2],
        })
    }

    pub fn init(&mut self) -> Result<()> {
        for i in 0..self.module_count {
            self.test_mode(i, false)?;
            self.send_command(i, Command::ScanLimit, 0x07)?;
            self.send_command(i, Command::DecodeMode, 0)?;
            self.clear(i)?;
        }
        Ok(())
    }

    pub fn clear(&mut self, addr: u8) -> Result<()> {
        for i in 1..9 {
            self.write_raw(addr as usize, i, 0x00)?;
        }

        Ok(())
    }

    pub fn test_mode(&mut self, addr: u8, is_on: bool) -> Result<()> {
        if is_on {
            self.send_command(addr, Command::DisplayTest, 0x01)
        } else {
            self.send_command(addr, Command::DisplayTest, 0x00)
        }
    }

    pub fn power_on(&mut self) {
        for i in 0..self.module_count {
            self.write_raw(i as usize, Command::Power as u8, 0x01)
                .unwrap();
        }
    }

    pub fn power_on_at(&mut self, addr: u8) {
        self.send_command(addr, Command::Power, 1);
    }

    pub fn power_off(&mut self) {
        for i in 0..self.module_count {
            self.send_command(i, Command::Power, 0);
        }
    }

    pub fn power_off_at(&mut self, addr: u8) {
        self.send_command(addr, Command::Power, 0);
    }

    pub fn send_command(&mut self, addr: u8, command: Command, data: u8) -> Result<()> {
        self.write_raw(addr as usize, command as u8, data)

        // for board_n in 0..self.module_count {
        //     self.cs
        //         .set_low()
        //         .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        //     if board_n != addr {
        //         self.send_bits(0u8.to_bits())
        //             .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        //         self.send_bits(0u8.to_bits())
        //             .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        //     } else {
        //         self.send_bits((command as u8).to_bits())
        //             .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        //         self.send_bits((data as u8).to_bits())
        //             .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        //     }
        //     self.cs
        //         .set_high()
        //         .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;

        // }
        // Ok(())
    }

    // pub fn write_raw(&mut self, addr: u8, bits: &[u8; 8]) -> Result<()> {
    //     for row in 0..8u8 {
    //         for board_n in 0..self.module_count {
    //             self.cs
    //                 .set_low()
    //                 .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
    //             if board_n != addr {
    //                 self.send_bits(0u16.to_bits())
    //                     .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
    //             } else {
    //                 self.send_bits(row.to_bits())
    //                     .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
    //                 self.send_bits(bits[row as usize].to_bits())
    //                     .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
    //             }
    //             self.cs
    //                 .set_high()
    //                 .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
    //         }
    //     }
    //     Ok(())
    // }

    fn write_raw(&mut self, addr: usize, header: u8, data: u8) -> Result<()> {
        let offset = addr * 2;
        let max_bytes = self.module_count * 2;
        self.buffer = [0; 8 * 2];

        self.buffer[offset] = header;
        self.buffer[offset + 1] = data;

        self.cs
            .set_low()
            .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        for b in 0..max_bytes {
            let value = self.buffer[b as usize];

            for i in 0..8 {
                if value & (1 << (7 - i)) > 0 {
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

    fn send_bits(&mut self, bits: Vec<bool>) -> Result<()> {
        for b in bits {
            match b {
                true => self
                    .data
                    .set_high()
                    .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?,
                false => self
                    .data
                    .set_low()
                    .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?,
            }
            self.clk
                .set_high()
                .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
            self.clk
                .set_low()
                .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        }
        Ok(())
    }

    pub fn test(&mut self, sleep_time: Duration) -> anyhow::Result<()> {
        for mod_addr in 0..self.module_count {
            self.test_mode(mod_addr, true)
                .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;

            sleep(sleep_time);

            self.test_mode(mod_addr, false)
                .map_err(|_| anyhow!(format!("{}:{}", file!(), line!())))?;
        }
        Ok(())
    }

    pub fn test_symbol(&mut self) {
        let mut symbol_src = symbol_cache::cache_keys();
        symbol_src.sort();

        let mut symbol_vec = vec![symbol_src.iter().collect::<Vec<_>>()];

        for i in 1..self.module_count as usize {
            symbol_vec.push(
                symbol_src
                    .iter()
                    .skip(i)
                    .chain(symbol_src.iter().take(i))
                    .collect(),
            );
        }

        println!("{:#?}", symbol_vec);

        for o in 0..symbol_src.len() {
            for i in 0..self.module_count {
                println!("sending {} to {}", symbol_vec[i as usize][o], i);
                if let Err(e) = self.write_raw(
                    i as usize,
                    1,
                    symbol_cache::get_symbol(symbol_vec[i as usize][o])
                        .unwrap()
                        .get_u8_grid()[0],
                ) {
                    eprintln!(
                        "error in getting u8 grid of {}\n error: {:?}",
                        symbol_vec[i as usize][o], e
                    );
                }
            }
            println!("sleeping");
            sleep(Duration::from_millis(500));
        }
    }

    pub fn update(&mut self) {
        todo!()
    }
}
