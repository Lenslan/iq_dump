use std::ops::Range;
use std::sync::{LazyLock, Mutex};
use pyo3::pyclass;
use strum::Display;

#[derive(PartialEq, Eq, Debug, Display, Clone, Copy)]
pub enum Band {
    HB,
    LB
}

pub enum TestBand {
    HB(GainType),
    LB(GainType)
}

impl TestBand {
    pub fn return_gain_type(&self) -> GainType {
        match self {
            TestBand::HB(item) => {item.return_type()}
            TestBand::LB(item) => {item.return_type()}
        }
    }
}

pub enum GainType {
    Fem(Range<u8>),
    Lna(Range<u8>),
    Vga(Range<u8>)
}

impl GainType {
    pub fn return_iter(&self) -> Range<u8> {
        match self {
            GainType::Fem(item) => {item.clone()}
            GainType::Lna(item) => {item.clone()}
            GainType::Vga(item) => {item.clone()}
        }
    }

    fn return_type(&self) -> Self {
        use GainType::*;
        match self {
            Fem(_) => {Fem(0..0)}
            Lna(_) => {Lna(0..0)}
            Vga(_) => {Vga(0..0)}
        }
    }
}

pub struct GlobPhyNum {
    hb: u8,
    lb: u8,
}
static GLOB_PHY_NUM_INSTANCE: LazyLock<Mutex<GlobPhyNum>> = LazyLock::new(|| {
    Mutex::new(GlobPhyNum{hb: 1, lb: 0})
});

impl GlobPhyNum {
    pub fn hb() -> u8 {
        let num = GLOB_PHY_NUM_INSTANCE.lock().unwrap();
        num.hb
    }

    pub fn lb() -> u8 {
        let num = GLOB_PHY_NUM_INSTANCE.lock().unwrap();
        num.lb
    }

    pub fn add_hb() {
        let mut num = GLOB_PHY_NUM_INSTANCE.lock().unwrap();
        let maxnum = num.hb.max(num.lb);
        num.hb = maxnum+1;
    }

    pub fn add_lb() {
        let mut num = GLOB_PHY_NUM_INSTANCE.lock().unwrap();
        let maxnum = num.lb.max(num.hb);
        num.lb = maxnum+1;
    }
}


