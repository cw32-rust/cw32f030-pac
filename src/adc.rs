#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    start: Start,
    sqr: Sqr,
    cr2: Cr2,
    vth: Vth,
    vtl: Vtl,
    trigger: Trigger,
    result0: Result0,
    result1: Result1,
    result2: Result2,
    result3: Result3,
    resultacc: Resultacc,
    ier: Ier,
    icr: Icr,
    isr: Isr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - Control register1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x08 - desc START"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x0c - desc SQR"]
    #[inline(always)]
    pub const fn sqr(&self) -> &Sqr {
        &self.sqr
    }
    #[doc = "0x10 - Control register2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x14 - desc VTH"]
    #[inline(always)]
    pub const fn vth(&self) -> &Vth {
        &self.vth
    }
    #[doc = "0x18 - desc VTL"]
    #[inline(always)]
    pub const fn vtl(&self) -> &Vtl {
        &self.vtl
    }
    #[doc = "0x1c - desc TRIGGER"]
    #[inline(always)]
    pub const fn trigger(&self) -> &Trigger {
        &self.trigger
    }
    #[doc = "0x20 - desc RESULT0"]
    #[inline(always)]
    pub const fn result0(&self) -> &Result0 {
        &self.result0
    }
    #[doc = "0x24 - desc RESULT1"]
    #[inline(always)]
    pub const fn result1(&self) -> &Result1 {
        &self.result1
    }
    #[doc = "0x28 - desc RESULT2"]
    #[inline(always)]
    pub const fn result2(&self) -> &Result2 {
        &self.result2
    }
    #[doc = "0x2c - desc RESULT3"]
    #[inline(always)]
    pub const fn result3(&self) -> &Result3 {
        &self.result3
    }
    #[doc = "0x30 - desc RESULTACC"]
    #[inline(always)]
    pub const fn resultacc(&self) -> &Resultacc {
        &self.resultacc
    }
    #[doc = "0x34 - Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x38 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x3c - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
}
#[doc = "CR0 (rw) register accessor: Control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "Control register0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "START (rw) register accessor: desc START\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "desc START"]
pub mod start;
#[doc = "SQR (rw) register accessor: desc SQR\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr`] module"]
#[doc(alias = "SQR")]
pub type Sqr = crate::Reg<sqr::SqrSpec>;
#[doc = "desc SQR"]
pub mod sqr;
#[doc = "CR2 (rw) register accessor: Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control register2"]
pub mod cr2;
#[doc = "VTH (rw) register accessor: desc VTH\n\nYou can [`read`](crate::Reg::read) this register and get [`vth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vth`] module"]
#[doc(alias = "VTH")]
pub type Vth = crate::Reg<vth::VthSpec>;
#[doc = "desc VTH"]
pub mod vth;
#[doc = "VTL (rw) register accessor: desc VTL\n\nYou can [`read`](crate::Reg::read) this register and get [`vtl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vtl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtl`] module"]
#[doc(alias = "VTL")]
pub type Vtl = crate::Reg<vtl::VtlSpec>;
#[doc = "desc VTL"]
pub mod vtl;
#[doc = "TRIGGER (rw) register accessor: desc TRIGGER\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
#[doc(alias = "TRIGGER")]
pub type Trigger = crate::Reg<trigger::TriggerSpec>;
#[doc = "desc TRIGGER"]
pub mod trigger;
#[doc = "RESULT0 (r) register accessor: desc RESULT0\n\nYou can [`read`](crate::Reg::read) this register and get [`result0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result0`] module"]
#[doc(alias = "RESULT0")]
pub type Result0 = crate::Reg<result0::Result0Spec>;
#[doc = "desc RESULT0"]
pub mod result0;
#[doc = "RESULT1 (r) register accessor: desc RESULT1\n\nYou can [`read`](crate::Reg::read) this register and get [`result1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result1`] module"]
#[doc(alias = "RESULT1")]
pub type Result1 = crate::Reg<result1::Result1Spec>;
#[doc = "desc RESULT1"]
pub mod result1;
#[doc = "RESULT2 (r) register accessor: desc RESULT2\n\nYou can [`read`](crate::Reg::read) this register and get [`result2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result2`] module"]
#[doc(alias = "RESULT2")]
pub type Result2 = crate::Reg<result2::Result2Spec>;
#[doc = "desc RESULT2"]
pub mod result2;
#[doc = "RESULT3 (r) register accessor: desc RESULT3\n\nYou can [`read`](crate::Reg::read) this register and get [`result3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result3`] module"]
#[doc(alias = "RESULT3")]
pub type Result3 = crate::Reg<result3::Result3Spec>;
#[doc = "desc RESULT3"]
pub mod result3;
#[doc = "RESULTACC (r) register accessor: desc RESULTACC\n\nYou can [`read`](crate::Reg::read) this register and get [`resultacc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resultacc`] module"]
#[doc(alias = "RESULTACC")]
pub type Resultacc = crate::Reg<resultacc::ResultaccSpec>;
#[doc = "desc RESULTACC"]
pub mod resultacc;
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ICR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt status register"]
pub mod isr;
