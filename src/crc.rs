#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    _reserved1: [u8; 0x04],
    _reserved_1_dr8: [u8; 0x04],
    _reserved_2_result16: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x08 - Data register"]
    #[inline(always)]
    pub const fn dr8(&self) -> &Dr8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Data register"]
    #[inline(always)]
    pub const fn dr16(&self) -> &Dr16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Data register"]
    #[inline(always)]
    pub const fn dr32(&self) -> &Dr32 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Result register"]
    #[inline(always)]
    pub const fn result16(&self) -> &Result16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Result register"]
    #[inline(always)]
    pub const fn result32(&self) -> &Result32 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "DR32 (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr32`] module"]
#[doc(alias = "DR32")]
pub type Dr32 = crate::Reg<dr32::Dr32Spec>;
#[doc = "Data register"]
pub mod dr32;
#[doc = "DR16 (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr16`] module"]
#[doc(alias = "DR16")]
pub type Dr16 = crate::Reg<dr16::Dr16Spec>;
#[doc = "Data register"]
pub mod dr16;
#[doc = "DR8 (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`] module"]
#[doc(alias = "DR8")]
pub type Dr8 = crate::Reg<dr8::Dr8Spec>;
#[doc = "Data register"]
pub mod dr8;
#[doc = "RESULT32 (r) register accessor: Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result32`] module"]
#[doc(alias = "RESULT32")]
pub type Result32 = crate::Reg<result32::Result32Spec>;
#[doc = "Result register"]
pub mod result32;
#[doc = "RESULT16 (r) register accessor: Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result16`] module"]
#[doc(alias = "RESULT16")]
pub type Result16 = crate::Reg<result16::Result16Spec>;
#[doc = "Result register"]
pub mod result16;
