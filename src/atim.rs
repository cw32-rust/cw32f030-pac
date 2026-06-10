#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    arr: Arr,
    cnt: Cnt,
    _reserved2: [u8; 0x04],
    cr: Cr,
    isr: Isr,
    icr: Icr,
    mscr: Mscr,
    fltr: Fltr,
    trig: Trig,
    ch1cr: Ch1cr,
    ch2cr: Ch2cr,
    ch3cr: Ch3cr,
    dtr: Dtr,
    rcr: Rcr,
    _reserved13: [u8; 0x04],
    ch1ccra: Ch1ccra,
    ch1ccrb: Ch1ccrb,
    ch2ccra: Ch2ccra,
    ch2ccrb: Ch2ccrb,
    ch3ccra: Ch3ccra,
    ch3ccrb: Ch3ccrb,
    ch4ccr: Ch4ccr,
    ch4cr: Ch4cr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc ARR"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x04 - desc CNT"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x0c - desc CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x10 - desc ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x14 - desc ICR"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x18 - desc MSCR"]
    #[inline(always)]
    pub const fn mscr(&self) -> &Mscr {
        &self.mscr
    }
    #[doc = "0x1c - desc FLTR"]
    #[inline(always)]
    pub const fn fltr(&self) -> &Fltr {
        &self.fltr
    }
    #[doc = "0x20 - desc TRIG"]
    #[inline(always)]
    pub const fn trig(&self) -> &Trig {
        &self.trig
    }
    #[doc = "0x24 - desc CH1CR"]
    #[inline(always)]
    pub const fn ch1cr(&self) -> &Ch1cr {
        &self.ch1cr
    }
    #[doc = "0x28 - desc CH2CR"]
    #[inline(always)]
    pub const fn ch2cr(&self) -> &Ch2cr {
        &self.ch2cr
    }
    #[doc = "0x2c - desc CH3CR"]
    #[inline(always)]
    pub const fn ch3cr(&self) -> &Ch3cr {
        &self.ch3cr
    }
    #[doc = "0x30 - desc DTR"]
    #[inline(always)]
    pub const fn dtr(&self) -> &Dtr {
        &self.dtr
    }
    #[doc = "0x34 - desc RCR"]
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    #[doc = "0x3c - desc CH1CCRA"]
    #[inline(always)]
    pub const fn ch1ccra(&self) -> &Ch1ccra {
        &self.ch1ccra
    }
    #[doc = "0x40 - desc CH1CCRB"]
    #[inline(always)]
    pub const fn ch1ccrb(&self) -> &Ch1ccrb {
        &self.ch1ccrb
    }
    #[doc = "0x44 - desc CH2CCRA"]
    #[inline(always)]
    pub const fn ch2ccra(&self) -> &Ch2ccra {
        &self.ch2ccra
    }
    #[doc = "0x48 - desc CH2CCRB"]
    #[inline(always)]
    pub const fn ch2ccrb(&self) -> &Ch2ccrb {
        &self.ch2ccrb
    }
    #[doc = "0x4c - desc CH3CCRA"]
    #[inline(always)]
    pub const fn ch3ccra(&self) -> &Ch3ccra {
        &self.ch3ccra
    }
    #[doc = "0x50 - desc CH3CCRB"]
    #[inline(always)]
    pub const fn ch3ccrb(&self) -> &Ch3ccrb {
        &self.ch3ccrb
    }
    #[doc = "0x54 - desc CH4CCR"]
    #[inline(always)]
    pub const fn ch4ccr(&self) -> &Ch4ccr {
        &self.ch4ccr
    }
    #[doc = "0x58 - desc CH4CR"]
    #[inline(always)]
    pub const fn ch4cr(&self) -> &Ch4cr {
        &self.ch4cr
    }
}
#[doc = "ARR (rw) register accessor: desc ARR\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "desc ARR"]
pub mod arr;
#[doc = "CNT (rw) register accessor: desc CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "desc CNT"]
pub mod cnt;
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "ISR (r) register accessor: desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "desc ISR"]
pub mod isr;
#[doc = "ICR (rw) register accessor: desc ICR\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "MSCR (rw) register accessor: desc MSCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mscr`] module"]
#[doc(alias = "MSCR")]
pub type Mscr = crate::Reg<mscr::MscrSpec>;
#[doc = "desc MSCR"]
pub mod mscr;
#[doc = "FLTR (rw) register accessor: desc FLTR\n\nYou can [`read`](crate::Reg::read) this register and get [`fltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltr`] module"]
#[doc(alias = "FLTR")]
pub type Fltr = crate::Reg<fltr::FltrSpec>;
#[doc = "desc FLTR"]
pub mod fltr;
#[doc = "TRIG (rw) register accessor: desc TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig`] module"]
#[doc(alias = "TRIG")]
pub type Trig = crate::Reg<trig::TrigSpec>;
#[doc = "desc TRIG"]
pub mod trig;
#[doc = "CH1CR (rw) register accessor: desc CH1CR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cr`] module"]
#[doc(alias = "CH1CR")]
pub type Ch1cr = crate::Reg<ch1cr::Ch1crSpec>;
#[doc = "desc CH1CR"]
pub mod ch1cr;
#[doc = "CH2CR (rw) register accessor: desc CH2CR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cr`] module"]
#[doc(alias = "CH2CR")]
pub type Ch2cr = crate::Reg<ch2cr::Ch2crSpec>;
#[doc = "desc CH2CR"]
pub mod ch2cr;
#[doc = "CH3CR (rw) register accessor: desc CH3CR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cr`] module"]
#[doc(alias = "CH3CR")]
pub type Ch3cr = crate::Reg<ch3cr::Ch3crSpec>;
#[doc = "desc CH3CR"]
pub mod ch3cr;
#[doc = "DTR (rw) register accessor: desc DTR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtr`] module"]
#[doc(alias = "DTR")]
pub type Dtr = crate::Reg<dtr::DtrSpec>;
#[doc = "desc DTR"]
pub mod dtr;
#[doc = "RCR (rw) register accessor: desc RCR\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`] module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "desc RCR"]
pub mod rcr;
#[doc = "CH1CCRA (rw) register accessor: desc CH1CCRA\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ccra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ccra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ccra`] module"]
#[doc(alias = "CH1CCRA")]
pub type Ch1ccra = crate::Reg<ch1ccra::Ch1ccraSpec>;
#[doc = "desc CH1CCRA"]
pub mod ch1ccra;
#[doc = "CH1CCRB (rw) register accessor: desc CH1CCRB\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ccrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ccrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ccrb`] module"]
#[doc(alias = "CH1CCRB")]
pub type Ch1ccrb = crate::Reg<ch1ccrb::Ch1ccrbSpec>;
#[doc = "desc CH1CCRB"]
pub mod ch1ccrb;
#[doc = "CH2CCRA (rw) register accessor: desc CH2CCRA\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2ccra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2ccra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ccra`] module"]
#[doc(alias = "CH2CCRA")]
pub type Ch2ccra = crate::Reg<ch2ccra::Ch2ccraSpec>;
#[doc = "desc CH2CCRA"]
pub mod ch2ccra;
#[doc = "CH2CCRB (rw) register accessor: desc CH2CCRB\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2ccrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2ccrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ccrb`] module"]
#[doc(alias = "CH2CCRB")]
pub type Ch2ccrb = crate::Reg<ch2ccrb::Ch2ccrbSpec>;
#[doc = "desc CH2CCRB"]
pub mod ch2ccrb;
#[doc = "CH3CCRA (rw) register accessor: desc CH3CCRA\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ccra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ccra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ccra`] module"]
#[doc(alias = "CH3CCRA")]
pub type Ch3ccra = crate::Reg<ch3ccra::Ch3ccraSpec>;
#[doc = "desc CH3CCRA"]
pub mod ch3ccra;
#[doc = "CH3CCRB (rw) register accessor: desc CH3CCRB\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ccrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ccrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ccrb`] module"]
#[doc(alias = "CH3CCRB")]
pub type Ch3ccrb = crate::Reg<ch3ccrb::Ch3ccrbSpec>;
#[doc = "desc CH3CCRB"]
pub mod ch3ccrb;
#[doc = "CH4CCR (rw) register accessor: desc CH4CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4ccr`] module"]
#[doc(alias = "CH4CCR")]
pub type Ch4ccr = crate::Reg<ch4ccr::Ch4ccrSpec>;
#[doc = "desc CH4CCR"]
pub mod ch4ccr;
#[doc = "CH4CR (rw) register accessor: desc CH4CR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cr`] module"]
#[doc(alias = "CH4CR")]
pub type Ch4cr = crate::Reg<ch4cr::Ch4crSpec>;
#[doc = "desc CH4CR"]
pub mod ch4cr;
