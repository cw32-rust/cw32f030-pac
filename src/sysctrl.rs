#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    cr2: Cr2,
    ier: Ier,
    isr: Isr,
    icr: Icr,
    hsi: Hsi,
    hse: Hse,
    lsi: Lsi,
    lse: Lse,
    pll: Pll,
    debug: Debug,
    ahben: Ahben,
    apben2: Apben2,
    apben1: Apben1,
    _reserved15: [u8; 0x04],
    ahbrst: Ahbrst,
    apbrst2: Apbrst2,
    apbrst1: Apbrst1,
    resetflag: Resetflag,
    gtim1cap: Gtim1cap,
    gtim2cap: Gtim2cap,
    gtim3cap: Gtim3cap,
    gtim4cap: Gtim4cap,
    atimetr: Atimetr,
    gtimetr: Gtimetr,
    _reserved25: [u8; 0x04],
    timitr: Timitr,
    mco: Mco,
    irmod: Irmod,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Reg0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - Control Reg1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x08 - Control Reg2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x0c - Interupt Enable Reg"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x10 - Interupt Status Reg"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x14 - Interupt Clear Reg"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x18 - HSI Control Reg"]
    #[inline(always)]
    pub const fn hsi(&self) -> &Hsi {
        &self.hsi
    }
    #[doc = "0x1c - HSE Control Reg"]
    #[inline(always)]
    pub const fn hse(&self) -> &Hse {
        &self.hse
    }
    #[doc = "0x20 - LSI Control Reg"]
    #[inline(always)]
    pub const fn lsi(&self) -> &Lsi {
        &self.lsi
    }
    #[doc = "0x24 - LSE Control Reg"]
    #[inline(always)]
    pub const fn lse(&self) -> &Lse {
        &self.lse
    }
    #[doc = "0x28 - PLL Control Reg"]
    #[inline(always)]
    pub const fn pll(&self) -> &Pll {
        &self.pll
    }
    #[doc = "0x2c - Debug Control Reg"]
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
    #[doc = "0x30 - AHB Clock Control Reg"]
    #[inline(always)]
    pub const fn ahben(&self) -> &Ahben {
        &self.ahben
    }
    #[doc = "0x34 - APB Clock Control Reg2"]
    #[inline(always)]
    pub const fn apben2(&self) -> &Apben2 {
        &self.apben2
    }
    #[doc = "0x38 - APB Clock Control Reg1"]
    #[inline(always)]
    pub const fn apben1(&self) -> &Apben1 {
        &self.apben1
    }
    #[doc = "0x40 - AHB Reset Control Reg"]
    #[inline(always)]
    pub const fn ahbrst(&self) -> &Ahbrst {
        &self.ahbrst
    }
    #[doc = "0x44 - APB Reset Control Reg2"]
    #[inline(always)]
    pub const fn apbrst2(&self) -> &Apbrst2 {
        &self.apbrst2
    }
    #[doc = "0x48 - APB Reset Control Reg1"]
    #[inline(always)]
    pub const fn apbrst1(&self) -> &Apbrst1 {
        &self.apbrst1
    }
    #[doc = "0x4c - Reset Status Reg"]
    #[inline(always)]
    pub const fn resetflag(&self) -> &Resetflag {
        &self.resetflag
    }
    #[doc = "0x50 - GTIM1 CAP Control Reg"]
    #[inline(always)]
    pub const fn gtim1cap(&self) -> &Gtim1cap {
        &self.gtim1cap
    }
    #[doc = "0x54 - GTIM2 CAP Control Reg"]
    #[inline(always)]
    pub const fn gtim2cap(&self) -> &Gtim2cap {
        &self.gtim2cap
    }
    #[doc = "0x58 - GTIM3 CAP Control Reg"]
    #[inline(always)]
    pub const fn gtim3cap(&self) -> &Gtim3cap {
        &self.gtim3cap
    }
    #[doc = "0x5c - GTIM4 CAP Control Reg"]
    #[inline(always)]
    pub const fn gtim4cap(&self) -> &Gtim4cap {
        &self.gtim4cap
    }
    #[doc = "0x60 - ATIM ETR Control Reg"]
    #[inline(always)]
    pub const fn atimetr(&self) -> &Atimetr {
        &self.atimetr
    }
    #[doc = "0x64 - GTIM1-4 ETR Control Reg"]
    #[inline(always)]
    pub const fn gtimetr(&self) -> &Gtimetr {
        &self.gtimetr
    }
    #[doc = "0x6c - BTIMx GTIMx ATIM ITR Control Reg"]
    #[inline(always)]
    pub const fn timitr(&self) -> &Timitr {
        &self.timitr
    }
    #[doc = "0x70 - Master Clock Output Control Reg"]
    #[inline(always)]
    pub const fn mco(&self) -> &Mco {
        &self.mco
    }
    #[doc = "0x74 - IR MOD Control Reg"]
    #[inline(always)]
    pub const fn irmod(&self) -> &Irmod {
        &self.irmod
    }
}
#[doc = "CR0 (rw) register accessor: Control Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "Control Reg0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control Reg1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control Reg2"]
pub mod cr2;
#[doc = "IER (rw) register accessor: Interupt Enable Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interupt Enable Reg"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interupt Status Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interupt Status Reg"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interupt Clear Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interupt Clear Reg"]
pub mod icr;
#[doc = "HSI (rw) register accessor: HSI Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hsi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsi`] module"]
#[doc(alias = "HSI")]
pub type Hsi = crate::Reg<hsi::HsiSpec>;
#[doc = "HSI Control Reg"]
pub mod hsi;
#[doc = "HSE (rw) register accessor: HSE Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hse`] module"]
#[doc(alias = "HSE")]
pub type Hse = crate::Reg<hse::HseSpec>;
#[doc = "HSE Control Reg"]
pub mod hse;
#[doc = "LSI (rw) register accessor: LSI Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`lsi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsi`] module"]
#[doc(alias = "LSI")]
pub type Lsi = crate::Reg<lsi::LsiSpec>;
#[doc = "LSI Control Reg"]
pub mod lsi;
#[doc = "LSE (rw) register accessor: LSE Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`lse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lse`] module"]
#[doc(alias = "LSE")]
pub type Lse = crate::Reg<lse::LseSpec>;
#[doc = "LSE Control Reg"]
pub mod lse;
#[doc = "PLL (rw) register accessor: PLL Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`pll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll`] module"]
#[doc(alias = "PLL")]
pub type Pll = crate::Reg<pll::PllSpec>;
#[doc = "PLL Control Reg"]
pub mod pll;
#[doc = "DEBUG (rw) register accessor: Debug Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`] module"]
#[doc(alias = "DEBUG")]
pub type Debug = crate::Reg<debug::DebugSpec>;
#[doc = "Debug Control Reg"]
pub mod debug;
#[doc = "AHBEN (rw) register accessor: AHB Clock Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben`] module"]
#[doc(alias = "AHBEN")]
pub type Ahben = crate::Reg<ahben::AhbenSpec>;
#[doc = "AHB Clock Control Reg"]
pub mod ahben;
#[doc = "APBEN2 (rw) register accessor: APB Clock Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`apben2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apben2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apben2`] module"]
#[doc(alias = "APBEN2")]
pub type Apben2 = crate::Reg<apben2::Apben2Spec>;
#[doc = "APB Clock Control Reg2"]
pub mod apben2;
#[doc = "APBEN1 (rw) register accessor: APB Clock Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`apben1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apben1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apben1`] module"]
#[doc(alias = "APBEN1")]
pub type Apben1 = crate::Reg<apben1::Apben1Spec>;
#[doc = "APB Clock Control Reg1"]
pub mod apben1;
#[doc = "AHBRST (rw) register accessor: AHB Reset Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst`] module"]
#[doc(alias = "AHBRST")]
pub type Ahbrst = crate::Reg<ahbrst::AhbrstSpec>;
#[doc = "AHB Reset Control Reg"]
pub mod ahbrst;
#[doc = "APBRST2 (rw) register accessor: APB Reset Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrst2`] module"]
#[doc(alias = "APBRST2")]
pub type Apbrst2 = crate::Reg<apbrst2::Apbrst2Spec>;
#[doc = "APB Reset Control Reg2"]
pub mod apbrst2;
#[doc = "APBRST1 (rw) register accessor: APB Reset Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrst1`] module"]
#[doc(alias = "APBRST1")]
pub type Apbrst1 = crate::Reg<apbrst1::Apbrst1Spec>;
#[doc = "APB Reset Control Reg1"]
pub mod apbrst1;
#[doc = "RESETFLAG (rw) register accessor: Reset Status Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`resetflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetflag`] module"]
#[doc(alias = "RESETFLAG")]
pub type Resetflag = crate::Reg<resetflag::ResetflagSpec>;
#[doc = "Reset Status Reg"]
pub mod resetflag;
#[doc = "GTIM1CAP (rw) register accessor: GTIM1 CAP Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`gtim1cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtim1cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtim1cap`] module"]
#[doc(alias = "GTIM1CAP")]
pub type Gtim1cap = crate::Reg<gtim1cap::Gtim1capSpec>;
#[doc = "GTIM1 CAP Control Reg"]
pub mod gtim1cap;
#[doc = "GTIM2CAP (rw) register accessor: GTIM2 CAP Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`gtim2cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtim2cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtim2cap`] module"]
#[doc(alias = "GTIM2CAP")]
pub type Gtim2cap = crate::Reg<gtim2cap::Gtim2capSpec>;
#[doc = "GTIM2 CAP Control Reg"]
pub mod gtim2cap;
#[doc = "GTIM3CAP (rw) register accessor: GTIM3 CAP Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`gtim3cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtim3cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtim3cap`] module"]
#[doc(alias = "GTIM3CAP")]
pub type Gtim3cap = crate::Reg<gtim3cap::Gtim3capSpec>;
#[doc = "GTIM3 CAP Control Reg"]
pub mod gtim3cap;
#[doc = "GTIM4CAP (rw) register accessor: GTIM4 CAP Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`gtim4cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtim4cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtim4cap`] module"]
#[doc(alias = "GTIM4CAP")]
pub type Gtim4cap = crate::Reg<gtim4cap::Gtim4capSpec>;
#[doc = "GTIM4 CAP Control Reg"]
pub mod gtim4cap;
#[doc = "ATIMETR (rw) register accessor: ATIM ETR Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`atimetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atimetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atimetr`] module"]
#[doc(alias = "ATIMETR")]
pub type Atimetr = crate::Reg<atimetr::AtimetrSpec>;
#[doc = "ATIM ETR Control Reg"]
pub mod atimetr;
#[doc = "GTIMETR (rw) register accessor: GTIM1-4 ETR Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`gtimetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtimetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtimetr`] module"]
#[doc(alias = "GTIMETR")]
pub type Gtimetr = crate::Reg<gtimetr::GtimetrSpec>;
#[doc = "GTIM1-4 ETR Control Reg"]
pub mod gtimetr;
#[doc = "TIMITR (rw) register accessor: BTIMx GTIMx ATIM ITR Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`timitr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timitr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timitr`] module"]
#[doc(alias = "TIMITR")]
pub type Timitr = crate::Reg<timitr::TimitrSpec>;
#[doc = "BTIMx GTIMx ATIM ITR Control Reg"]
pub mod timitr;
#[doc = "MCO (rw) register accessor: Master Clock Output Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`mco::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mco`] module"]
#[doc(alias = "MCO")]
pub type Mco = crate::Reg<mco::McoSpec>;
#[doc = "Master Clock Output Control Reg"]
pub mod mco;
#[doc = "IRMOD (rw) register accessor: IR MOD Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`irmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irmod`] module"]
#[doc(alias = "IRMOD")]
pub type Irmod = crate::Reg<irmod::IrmodSpec>;
#[doc = "IR MOD Control Reg"]
pub mod irmod;
