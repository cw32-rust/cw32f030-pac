#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    arr: Arr,
    cnt: Cnt,
    _reserved2: [u8; 0x04],
    acr: Acr,
    bcr: Bcr,
    ier: Ier,
    isr: Isr,
    icr: Icr,
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x00 - Auto Reload Register"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x04 - Counter Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x0c - Advanced Control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x10 - Base Control register"]
    #[inline(always)]
    pub const fn bcr(&self) -> &Bcr {
        &self.bcr
    }
    #[doc = "0x14 - Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x18 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x1c - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x20 - DMA control register"]
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
#[doc = "ACR (rw) register accessor: Advanced Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Advanced Control register"]
pub mod acr;
#[doc = "BCR (rw) register accessor: Base Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`] module"]
#[doc(alias = "BCR")]
pub type Bcr = crate::Reg<bcr::BcrSpec>;
#[doc = "Base Control register"]
pub mod bcr;
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "DMA (rw) register accessor: DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`] module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "DMA control register"]
pub mod dma;
