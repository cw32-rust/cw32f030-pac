#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    sr: Sr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register1"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - Control register2"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
}
#[doc = "CR0 (rw) register accessor: Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "Control register1"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control register2"]
pub mod cr1;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
