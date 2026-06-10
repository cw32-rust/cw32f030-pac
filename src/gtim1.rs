#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    arr: Arr,
    cnt: Cnt,
    cmmr: Cmmr,
    etr: Etr,
    cr0: Cr0,
    ier: Ier,
    isr: Isr,
    icr: Icr,
    ccr1: Ccr1,
    ccr2: Ccr2,
    ccr3: Ccr3,
    ccr4: Ccr4,
    cr1: Cr1,
    _reserved13: [u8; 0x0c],
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x300 - Auto Reload Register"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x304 - Counter Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x308 - Capture compare control Register"]
    #[inline(always)]
    pub const fn cmmr(&self) -> &Cmmr {
        &self.cmmr
    }
    #[doc = "0x30c - ETR Control register"]
    #[inline(always)]
    pub const fn etr(&self) -> &Etr {
        &self.etr
    }
    #[doc = "0x310 - Control register0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x314 - Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x318 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x31c - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x320 - capture compare register"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x324 - capture compare register"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x328 - capture compare register"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x32c - capture compare register"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    #[doc = "0x330 - Control register1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x340 - DMA Control register"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
}
#[doc = "ARR (rw) register accessor: Auto Reload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "Auto Reload Register"]
pub mod arr;
#[doc = "CNT (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter Register"]
pub mod cnt;
#[doc = "CMMR (rw) register accessor: Capture compare control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmmr`] module"]
#[doc(alias = "CMMR")]
pub type Cmmr = crate::Reg<cmmr::CmmrSpec>;
#[doc = "Capture compare control Register"]
pub mod cmmr;
#[doc = "ETR (rw) register accessor: ETR Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`etr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etr`] module"]
#[doc(alias = "ETR")]
pub type Etr = crate::Reg<etr::EtrSpec>;
#[doc = "ETR Control register"]
pub mod etr;
#[doc = "CR0 (rw) register accessor: Control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "Control register0"]
pub mod cr0;
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ISR (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "CCR1 (rw) register accessor: capture compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "capture compare register"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: capture compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "capture compare register"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: capture compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::Ccr3Spec>;
#[doc = "capture compare register"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: capture compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`] module"]
#[doc(alias = "CCR4")]
pub type Ccr4 = crate::Reg<ccr4::Ccr4Spec>;
#[doc = "capture compare register"]
pub mod ccr4;
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "DMA (rw) register accessor: DMA Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`] module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "DMA Control register"]
pub mod dma;
