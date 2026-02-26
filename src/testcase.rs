use std::fs::OpenOptions;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use crate::client::Dut;
use crate::config::{Band, GainType, GlobPhyNum, TestBand};

pub trait TestCase {
    fn traverse(&self) -> impl Iterator<Item=u8>;
    fn get_band(&self) -> Band;

    fn get_gain_type(&self) -> GainType;

    fn run_single_fem(&self, idx:u8, dut: &mut Dut) -> anyhow::Result<()> {
        let band = self.get_band();
        let channel = GlobPhyNum::get_channel();
        dut.fix_gain(band, idx, 0, 0)?;
        let iq_name = format!("{}_iq_{}_{}_0_00.txt", band, channel, idx);
        dut.dump_iq(band, iq_name.clone())?;
        dut.copy_files(iq_name)?;
        dut.del_files()?;
        Ok(())
    }
    fn run_single_lna(&self, idx:u8, dut: &mut Dut) -> anyhow::Result<()> {
        let band = self.get_band();
        let channel = GlobPhyNum::get_channel();
        dut.fix_gain(band, 0, idx, 0)?;
        let iq_name = format!("{}_iq_{}_0_{}_00.txt", band, channel, idx);
        dut.dump_iq(band, iq_name.clone())?;
        dut.copy_files(iq_name)?;
        dut.del_files()?;
        Ok(())
    }
    fn run_single_vga(&self, idx:u8, dut: &mut Dut) -> anyhow::Result<()> {
        let band = self.get_band();
        let channel = GlobPhyNum::get_channel();
        dut.fix_gain(band, 0, 0, idx)?;
        let iq_name = format!("{}_iq_{}_0_0_{:02}.txt", band, channel, idx);
        dut.dump_iq(band, iq_name.clone())?;
        dut.copy_files(iq_name)?;
        dut.del_files()?;
        // dut.devmem(0x30d0004c, 0x80000000)?;
        // sleep(Duration::from_millis(1000));
        // let res_path1 = dut.read_reg(0x30d0004c)?;
        //
        // dut.devmem(0x30d00050, 0x80000000)?;
        // sleep(Duration::from_millis(1000));
        // let res_path2 = dut.read_reg(0x30d00050)?;
        //
        // let mut file = OpenOptions::new()
        //     .create(true)
        //     .append(true)
        //     .open("result.txt")?;
        // writeln!(file, "vga:{}, path1_dc:{}, path2_dc:{}", idx, res_path1, res_path2)?;
        Ok(())
    }

    fn run_test(&self, dut: &mut Dut) {
        match self.get_gain_type() {
            GainType::Fem(_) => {
                self.traverse()
                    .for_each(|x| {
                        match self.run_single_fem(x, dut) {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Run test Error: {}", e)
                            }
                        }
                    })
            }
            GainType::Lna(_) => {
                self.traverse()
                    .for_each(|x| {
                        match self.run_single_lna(x, dut) {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Run test Error: {}", e)
                            }
                        }
                    })
            }
            GainType::Vga(_) => {
                self.traverse()
                    .for_each(|x| {
                        match self.run_single_vga(x, dut) {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Run test Error: {}", e)
                            }
                        }
                    })
            }
        }
    }

}

impl TestCase for TestBand {
    fn traverse(&self) -> impl Iterator<Item=u8> {
        match self {
            TestBand::HB(item) => {
                item.return_iter()
            }
            TestBand::LB(item) => {
                item.return_iter()
            }
        }
    }
    fn get_band(&self) -> Band {
        match self {
            TestBand::HB(_) => {
                Band::HB
            }
            TestBand::LB(_) => {
                Band::LB
            }
        }
    }

    fn get_gain_type(&self) -> GainType {
        self.return_gain_type()
    }
}