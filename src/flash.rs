#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    pagelock: Pagelock,
    _reserved3: [u8; 0x14],
    ier: Ier,
    isr: Isr,
    icr: Icr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - Control register2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - Page Write Erase Lock"]
    #[inline(always)]
    pub const fn pagelock(&self) -> &Pagelock {
        &self.pagelock
    }
    #[doc = "0x20 - Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x24 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x28 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
}
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control register2"]
pub mod cr2;
#[doc = "PAGELOCK (rw) register accessor: Page Write Erase Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`pagelock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pagelock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pagelock`] module"]
#[doc(alias = "PAGELOCK")]
pub type Pagelock = crate::Reg<pagelock::PagelockSpec>;
#[doc = "Page Write Erase Lock"]
pub mod pagelock;
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
