#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    cnt: Cnt,
    srcaddr: Srcaddr,
    dstaddr: Dstaddr,
    trig: Trig,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel.y control and status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - Channel.y counter register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x08 - Channel.y source address register"]
    #[inline(always)]
    pub const fn srcaddr(&self) -> &Srcaddr {
        &self.srcaddr
    }
    #[doc = "0x0c - Channel.y destination address register"]
    #[inline(always)]
    pub const fn dstaddr(&self) -> &Dstaddr {
        &self.dstaddr
    }
    #[doc = "0x10 - Channel.y trigger register"]
    #[inline(always)]
    pub const fn trig(&self) -> &Trig {
        &self.trig
    }
}
#[doc = "CSR (rw) register accessor: Channel.y control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Channel.y control and status register"]
pub mod csr;
#[doc = "CNT (rw) register accessor: Channel.y counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Channel.y counter register"]
pub mod cnt;
#[doc = "SRCADDR (rw) register accessor: Channel.y source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr`] module"]
#[doc(alias = "SRCADDR")]
pub type Srcaddr = crate::Reg<srcaddr::SrcaddrSpec>;
#[doc = "Channel.y source address register"]
pub mod srcaddr;
#[doc = "DSTADDR (rw) register accessor: Channel.y destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr`] module"]
#[doc(alias = "DSTADDR")]
pub type Dstaddr = crate::Reg<dstaddr::DstaddrSpec>;
#[doc = "Channel.y destination address register"]
pub mod dstaddr;
#[doc = "TRIG (rw) register accessor: Channel.y trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig`] module"]
#[doc(alias = "TRIG")]
pub type Trig = crate::Reg<trig::TrigSpec>;
#[doc = "Channel.y trigger register"]
pub mod trig;
