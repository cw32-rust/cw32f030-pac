#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    kr: Kr,
    cr: Cr,
    arr: Arr,
    sr: Sr,
    winr: Winr,
    _reserved5: [u8; 0x10],
    cnt: Cnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Key register"]
    #[inline(always)]
    pub const fn kr(&self) -> &Kr {
        &self.kr
    }
    #[doc = "0x04 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x08 - Auto reload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - Window register"]
    #[inline(always)]
    pub const fn winr(&self) -> &Winr {
        &self.winr
    }
    #[doc = "0x24 - counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
}
#[doc = "KR (w) register accessor: Key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`] module"]
#[doc(alias = "KR")]
pub type Kr = crate::Reg<kr::KrSpec>;
#[doc = "Key register"]
pub mod kr;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "ARR (rw) register accessor: Auto reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "Auto reload register"]
pub mod arr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "WINR (rw) register accessor: Window register\n\nYou can [`read`](crate::Reg::read) this register and get [`winr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winr`] module"]
#[doc(alias = "WINR")]
pub type Winr = crate::Reg<winr::WinrSpec>;
#[doc = "Window register"]
pub mod winr;
#[doc = "CNT (r) register accessor: counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "counter"]
pub mod cnt;
