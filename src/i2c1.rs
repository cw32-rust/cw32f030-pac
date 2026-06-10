#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    brren: Brren,
    brr: Brr,
    cr: Cr,
    dr: Dr,
    addr0: Addr0,
    stat: Stat,
    _reserved6: [u8; 0x08],
    addr1: Addr1,
    addr2: Addr2,
    match_: Match,
}
impl RegisterBlock {
    #[doc = "0x00 - desc BRREN"]
    #[inline(always)]
    pub const fn brren(&self) -> &Brren {
        &self.brren
    }
    #[doc = "0x04 - desc BRR"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x10 - Slave Addrress0"]
    #[inline(always)]
    pub const fn addr0(&self) -> &Addr0 {
        &self.addr0
    }
    #[doc = "0x14 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x20 - Slave Addrress1"]
    #[inline(always)]
    pub const fn addr1(&self) -> &Addr1 {
        &self.addr1
    }
    #[doc = "0x24 - Slave Addrress2"]
    #[inline(always)]
    pub const fn addr2(&self) -> &Addr2 {
        &self.addr2
    }
    #[doc = "0x28 - Slave Addrress match flag"]
    #[inline(always)]
    pub const fn match_(&self) -> &Match {
        &self.match_
    }
}
#[doc = "BRREN (rw) register accessor: desc BRREN\n\nYou can [`read`](crate::Reg::read) this register and get [`brren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brren`] module"]
#[doc(alias = "BRREN")]
pub type Brren = crate::Reg<brren::BrrenSpec>;
#[doc = "desc BRREN"]
pub mod brren;
#[doc = "BRR (rw) register accessor: desc BRR\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "desc BRR"]
pub mod brr;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "DR (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "Data register"]
pub mod dr;
#[doc = "ADDR0 (rw) register accessor: Slave Addrress0\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0`] module"]
#[doc(alias = "ADDR0")]
pub type Addr0 = crate::Reg<addr0::Addr0Spec>;
#[doc = "Slave Addrress0"]
pub mod addr0;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "ADDR1 (rw) register accessor: Slave Addrress1\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1`] module"]
#[doc(alias = "ADDR1")]
pub type Addr1 = crate::Reg<addr1::Addr1Spec>;
#[doc = "Slave Addrress1"]
pub mod addr1;
#[doc = "ADDR2 (rw) register accessor: Slave Addrress2\n\nYou can [`read`](crate::Reg::read) this register and get [`addr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr2`] module"]
#[doc(alias = "ADDR2")]
pub type Addr2 = crate::Reg<addr2::Addr2Spec>;
#[doc = "Slave Addrress2"]
pub mod addr2;
#[doc = "MATCH (r) register accessor: Slave Addrress match flag\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match_`] module"]
#[doc(alias = "MATCH")]
pub type Match = crate::Reg<match_::MatchSpec>;
#[doc = "Slave Addrress match flag"]
pub mod match_;
