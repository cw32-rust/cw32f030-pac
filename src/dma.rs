#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    icr: Icr,
    _reserved2: [u8; 0x18],
    csr1: Csr1,
    cnt1: Cnt1,
    srcaddr1: Srcaddr1,
    dstaddr1: Dstaddr1,
    trig1: Trig1,
    _reserved7: [u8; 0x0c],
    csr2: Csr2,
    cnt2: Cnt2,
    srcaddr2: Srcaddr2,
    dstaddr2: Dstaddr2,
    trig2: Trig2,
    _reserved12: [u8; 0x0c],
    csr3: Csr3,
    cnt3: Cnt3,
    srcaddr3: Srcaddr3,
    dstaddr3: Dstaddr3,
    trig3: Trig3,
    _reserved17: [u8; 0x0c],
    csr4: Csr4,
    cnt4: Cnt4,
    srcaddr4: Srcaddr4,
    dstaddr4: Dstaddr4,
    trig4: Trig4,
    _reserved22: [u8; 0x0c],
    csr5: Csr5,
    cnt5: Cnt5,
    srcaddr5: Srcaddr5,
    dstaddr5: Dstaddr5,
    trig5: Trig5,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x20 - Channel1 control and status register"]
    #[inline(always)]
    pub const fn csr1(&self) -> &Csr1 {
        &self.csr1
    }
    #[doc = "0x24 - Channel1 counter register"]
    #[inline(always)]
    pub const fn cnt1(&self) -> &Cnt1 {
        &self.cnt1
    }
    #[doc = "0x28 - Channel1 source address register"]
    #[inline(always)]
    pub const fn srcaddr1(&self) -> &Srcaddr1 {
        &self.srcaddr1
    }
    #[doc = "0x2c - Channel1 destination address register"]
    #[inline(always)]
    pub const fn dstaddr1(&self) -> &Dstaddr1 {
        &self.dstaddr1
    }
    #[doc = "0x30 - Channel1 trigger register"]
    #[inline(always)]
    pub const fn trig1(&self) -> &Trig1 {
        &self.trig1
    }
    #[doc = "0x40 - Channel2 control and status register"]
    #[inline(always)]
    pub const fn csr2(&self) -> &Csr2 {
        &self.csr2
    }
    #[doc = "0x44 - Channel2 counter register"]
    #[inline(always)]
    pub const fn cnt2(&self) -> &Cnt2 {
        &self.cnt2
    }
    #[doc = "0x48 - Channel2 source address register"]
    #[inline(always)]
    pub const fn srcaddr2(&self) -> &Srcaddr2 {
        &self.srcaddr2
    }
    #[doc = "0x4c - Channel2 destination address register"]
    #[inline(always)]
    pub const fn dstaddr2(&self) -> &Dstaddr2 {
        &self.dstaddr2
    }
    #[doc = "0x50 - Channel2 trigger register"]
    #[inline(always)]
    pub const fn trig2(&self) -> &Trig2 {
        &self.trig2
    }
    #[doc = "0x60 - Channel3 control and status register"]
    #[inline(always)]
    pub const fn csr3(&self) -> &Csr3 {
        &self.csr3
    }
    #[doc = "0x64 - Channel3 counter register"]
    #[inline(always)]
    pub const fn cnt3(&self) -> &Cnt3 {
        &self.cnt3
    }
    #[doc = "0x68 - Channel3 source address register"]
    #[inline(always)]
    pub const fn srcaddr3(&self) -> &Srcaddr3 {
        &self.srcaddr3
    }
    #[doc = "0x6c - Channel3 destination address register"]
    #[inline(always)]
    pub const fn dstaddr3(&self) -> &Dstaddr3 {
        &self.dstaddr3
    }
    #[doc = "0x70 - Channel3 trigger register"]
    #[inline(always)]
    pub const fn trig3(&self) -> &Trig3 {
        &self.trig3
    }
    #[doc = "0x80 - Channel4 control and status register"]
    #[inline(always)]
    pub const fn csr4(&self) -> &Csr4 {
        &self.csr4
    }
    #[doc = "0x84 - Channel4 counter register"]
    #[inline(always)]
    pub const fn cnt4(&self) -> &Cnt4 {
        &self.cnt4
    }
    #[doc = "0x88 - Channel4 source address register"]
    #[inline(always)]
    pub const fn srcaddr4(&self) -> &Srcaddr4 {
        &self.srcaddr4
    }
    #[doc = "0x8c - Channel4 destination address register"]
    #[inline(always)]
    pub const fn dstaddr4(&self) -> &Dstaddr4 {
        &self.dstaddr4
    }
    #[doc = "0x90 - Channel4 trigger register"]
    #[inline(always)]
    pub const fn trig4(&self) -> &Trig4 {
        &self.trig4
    }
    #[doc = "0xa0 - Channel5 control and status register"]
    #[inline(always)]
    pub const fn csr5(&self) -> &Csr5 {
        &self.csr5
    }
    #[doc = "0xa4 - Channel5 counter register"]
    #[inline(always)]
    pub const fn cnt5(&self) -> &Cnt5 {
        &self.cnt5
    }
    #[doc = "0xa8 - Channel5 source address register"]
    #[inline(always)]
    pub const fn srcaddr5(&self) -> &Srcaddr5 {
        &self.srcaddr5
    }
    #[doc = "0xac - Channel5 destination address register"]
    #[inline(always)]
    pub const fn dstaddr5(&self) -> &Dstaddr5 {
        &self.dstaddr5
    }
    #[doc = "0xb0 - Channel5 trigger register"]
    #[inline(always)]
    pub const fn trig5(&self) -> &Trig5 {
        &self.trig5
    }
}
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
#[doc = "CSR1 (rw) register accessor: Channel1 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`] module"]
#[doc(alias = "CSR1")]
pub type Csr1 = crate::Reg<csr1::Csr1Spec>;
#[doc = "Channel1 control and status register"]
pub mod csr1;
#[doc = "CNT1 (rw) register accessor: Channel1 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt1`] module"]
#[doc(alias = "CNT1")]
pub type Cnt1 = crate::Reg<cnt1::Cnt1Spec>;
#[doc = "Channel1 counter register"]
pub mod cnt1;
#[doc = "SRCADDR1 (rw) register accessor: Channel1 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr1`] module"]
#[doc(alias = "SRCADDR1")]
pub type Srcaddr1 = crate::Reg<srcaddr1::Srcaddr1Spec>;
#[doc = "Channel1 source address register"]
pub mod srcaddr1;
#[doc = "DSTADDR1 (rw) register accessor: Channel1 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr1`] module"]
#[doc(alias = "DSTADDR1")]
pub type Dstaddr1 = crate::Reg<dstaddr1::Dstaddr1Spec>;
#[doc = "Channel1 destination address register"]
pub mod dstaddr1;
#[doc = "TRIG1 (rw) register accessor: Channel1 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig1`] module"]
#[doc(alias = "TRIG1")]
pub type Trig1 = crate::Reg<trig1::Trig1Spec>;
#[doc = "Channel1 trigger register"]
pub mod trig1;
#[doc = "CSR2 (rw) register accessor: Channel2 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`] module"]
#[doc(alias = "CSR2")]
pub type Csr2 = crate::Reg<csr2::Csr2Spec>;
#[doc = "Channel2 control and status register"]
pub mod csr2;
#[doc = "CNT2 (rw) register accessor: Channel2 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt2`] module"]
#[doc(alias = "CNT2")]
pub type Cnt2 = crate::Reg<cnt2::Cnt2Spec>;
#[doc = "Channel2 counter register"]
pub mod cnt2;
#[doc = "SRCADDR2 (rw) register accessor: Channel2 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr2`] module"]
#[doc(alias = "SRCADDR2")]
pub type Srcaddr2 = crate::Reg<srcaddr2::Srcaddr2Spec>;
#[doc = "Channel2 source address register"]
pub mod srcaddr2;
#[doc = "DSTADDR2 (rw) register accessor: Channel2 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr2`] module"]
#[doc(alias = "DSTADDR2")]
pub type Dstaddr2 = crate::Reg<dstaddr2::Dstaddr2Spec>;
#[doc = "Channel2 destination address register"]
pub mod dstaddr2;
#[doc = "TRIG2 (rw) register accessor: Channel2 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig2`] module"]
#[doc(alias = "TRIG2")]
pub type Trig2 = crate::Reg<trig2::Trig2Spec>;
#[doc = "Channel2 trigger register"]
pub mod trig2;
#[doc = "CSR3 (rw) register accessor: Channel3 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr3`] module"]
#[doc(alias = "CSR3")]
pub type Csr3 = crate::Reg<csr3::Csr3Spec>;
#[doc = "Channel3 control and status register"]
pub mod csr3;
#[doc = "CNT3 (rw) register accessor: Channel3 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt3`] module"]
#[doc(alias = "CNT3")]
pub type Cnt3 = crate::Reg<cnt3::Cnt3Spec>;
#[doc = "Channel3 counter register"]
pub mod cnt3;
#[doc = "SRCADDR3 (rw) register accessor: Channel3 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr3`] module"]
#[doc(alias = "SRCADDR3")]
pub type Srcaddr3 = crate::Reg<srcaddr3::Srcaddr3Spec>;
#[doc = "Channel3 source address register"]
pub mod srcaddr3;
#[doc = "DSTADDR3 (rw) register accessor: Channel3 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr3`] module"]
#[doc(alias = "DSTADDR3")]
pub type Dstaddr3 = crate::Reg<dstaddr3::Dstaddr3Spec>;
#[doc = "Channel3 destination address register"]
pub mod dstaddr3;
#[doc = "TRIG3 (rw) register accessor: Channel3 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig3`] module"]
#[doc(alias = "TRIG3")]
pub type Trig3 = crate::Reg<trig3::Trig3Spec>;
#[doc = "Channel3 trigger register"]
pub mod trig3;
#[doc = "CSR4 (rw) register accessor: Channel4 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr4`] module"]
#[doc(alias = "CSR4")]
pub type Csr4 = crate::Reg<csr4::Csr4Spec>;
#[doc = "Channel4 control and status register"]
pub mod csr4;
#[doc = "CNT4 (rw) register accessor: Channel4 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt4`] module"]
#[doc(alias = "CNT4")]
pub type Cnt4 = crate::Reg<cnt4::Cnt4Spec>;
#[doc = "Channel4 counter register"]
pub mod cnt4;
#[doc = "SRCADDR4 (rw) register accessor: Channel4 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr4`] module"]
#[doc(alias = "SRCADDR4")]
pub type Srcaddr4 = crate::Reg<srcaddr4::Srcaddr4Spec>;
#[doc = "Channel4 source address register"]
pub mod srcaddr4;
#[doc = "DSTADDR4 (rw) register accessor: Channel4 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr4`] module"]
#[doc(alias = "DSTADDR4")]
pub type Dstaddr4 = crate::Reg<dstaddr4::Dstaddr4Spec>;
#[doc = "Channel4 destination address register"]
pub mod dstaddr4;
#[doc = "TRIG4 (rw) register accessor: Channel4 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig4`] module"]
#[doc(alias = "TRIG4")]
pub type Trig4 = crate::Reg<trig4::Trig4Spec>;
#[doc = "Channel4 trigger register"]
pub mod trig4;
#[doc = "CSR5 (rw) register accessor: Channel5 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr5`] module"]
#[doc(alias = "CSR5")]
pub type Csr5 = crate::Reg<csr5::Csr5Spec>;
#[doc = "Channel5 control and status register"]
pub mod csr5;
#[doc = "CNT5 (rw) register accessor: Channel5 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt5`] module"]
#[doc(alias = "CNT5")]
pub type Cnt5 = crate::Reg<cnt5::Cnt5Spec>;
#[doc = "Channel5 counter register"]
pub mod cnt5;
#[doc = "SRCADDR5 (rw) register accessor: Channel5 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr5`] module"]
#[doc(alias = "SRCADDR5")]
pub type Srcaddr5 = crate::Reg<srcaddr5::Srcaddr5Spec>;
#[doc = "Channel5 source address register"]
pub mod srcaddr5;
#[doc = "DSTADDR5 (rw) register accessor: Channel5 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr5`] module"]
#[doc(alias = "DSTADDR5")]
pub type Dstaddr5 = crate::Reg<dstaddr5::Dstaddr5Spec>;
#[doc = "Channel5 destination address register"]
pub mod dstaddr5;
#[doc = "TRIG5 (rw) register accessor: Channel5 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig5`] module"]
#[doc(alias = "TRIG5")]
pub type Trig5 = crate::Reg<trig5::Trig5Spec>;
#[doc = "Channel5 trigger register"]
pub mod trig5;
