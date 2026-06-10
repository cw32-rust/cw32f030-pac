#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dir: Dir,
    opendrain: Opendrain,
    speed: Speed,
    pdr: Pdr,
    pur: Pur,
    afrh: Afrh,
    afrl: Afrl,
    analog: Analog,
    driver: Driver,
    riseie: Riseie,
    fallie: Fallie,
    highie: Highie,
    lowie: Lowie,
    isr: Isr,
    icr: Icr,
    lock: Lock,
    filter: Filter,
    _reserved17: [u8; 0x0c],
    idr: Idr,
    _reserved_18_odr: [u8; 0x04],
    brr: Brr,
    bsrr: Bsrr,
    tog: Tog,
}
impl RegisterBlock {
    #[doc = "0x00 - desc DIR"]
    #[inline(always)]
    pub const fn dir(&self) -> &Dir {
        &self.dir
    }
    #[doc = "0x04 - desc OPENDRAIN"]
    #[inline(always)]
    pub const fn opendrain(&self) -> &Opendrain {
        &self.opendrain
    }
    #[doc = "0x08 - desc SPEED"]
    #[inline(always)]
    pub const fn speed(&self) -> &Speed {
        &self.speed
    }
    #[doc = "0x0c - desc PDR"]
    #[inline(always)]
    pub const fn pdr(&self) -> &Pdr {
        &self.pdr
    }
    #[doc = "0x10 - desc PUR"]
    #[inline(always)]
    pub const fn pur(&self) -> &Pur {
        &self.pur
    }
    #[doc = "0x14 - desc AFRH"]
    #[inline(always)]
    pub const fn afrh(&self) -> &Afrh {
        &self.afrh
    }
    #[doc = "0x18 - desc AFRL"]
    #[inline(always)]
    pub const fn afrl(&self) -> &Afrl {
        &self.afrl
    }
    #[doc = "0x1c - desc ANALOG"]
    #[inline(always)]
    pub const fn analog(&self) -> &Analog {
        &self.analog
    }
    #[doc = "0x20 - desc DRIVER"]
    #[inline(always)]
    pub const fn driver(&self) -> &Driver {
        &self.driver
    }
    #[doc = "0x24 - Interrupt enable register"]
    #[inline(always)]
    pub const fn riseie(&self) -> &Riseie {
        &self.riseie
    }
    #[doc = "0x28 - Interrupt enable register"]
    #[inline(always)]
    pub const fn fallie(&self) -> &Fallie {
        &self.fallie
    }
    #[doc = "0x2c - Interrupt enable register"]
    #[inline(always)]
    pub const fn highie(&self) -> &Highie {
        &self.highie
    }
    #[doc = "0x30 - Interrupt enable register"]
    #[inline(always)]
    pub const fn lowie(&self) -> &Lowie {
        &self.lowie
    }
    #[doc = "0x34 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x38 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x3c - desc LOCK"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x40 - desc FILTER"]
    #[inline(always)]
    pub const fn filter(&self) -> &Filter {
        &self.filter
    }
    #[doc = "0x50 - desc IDR"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x54 - desc ODRLOWBYTE"]
    #[inline(always)]
    pub const fn odrlowbyte(&self) -> &Odrlowbyte {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - desc ODR"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x55 - desc ODRHIGHBYTE"]
    #[inline(always)]
    pub const fn odrhighbyte(&self) -> &Odrhighbyte {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(85).cast() }
    }
    #[doc = "0x58 - desc BRR"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x5c - desc BSRR"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &Bsrr {
        &self.bsrr
    }
    #[doc = "0x60 - desc TOG"]
    #[inline(always)]
    pub const fn tog(&self) -> &Tog {
        &self.tog
    }
}
#[doc = "DIR (rw) register accessor: desc DIR\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`] module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "desc DIR"]
pub mod dir;
#[doc = "OPENDRAIN (rw) register accessor: desc OPENDRAIN\n\nYou can [`read`](crate::Reg::read) this register and get [`opendrain::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opendrain::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opendrain`] module"]
#[doc(alias = "OPENDRAIN")]
pub type Opendrain = crate::Reg<opendrain::OpendrainSpec>;
#[doc = "desc OPENDRAIN"]
pub mod opendrain;
#[doc = "SPEED (rw) register accessor: desc SPEED\n\nYou can [`read`](crate::Reg::read) this register and get [`speed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`speed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@speed`] module"]
#[doc(alias = "SPEED")]
pub type Speed = crate::Reg<speed::SpeedSpec>;
#[doc = "desc SPEED"]
pub mod speed;
#[doc = "PDR (rw) register accessor: desc PDR\n\nYou can [`read`](crate::Reg::read) this register and get [`pdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr`] module"]
#[doc(alias = "PDR")]
pub type Pdr = crate::Reg<pdr::PdrSpec>;
#[doc = "desc PDR"]
pub mod pdr;
#[doc = "PUR (rw) register accessor: desc PUR\n\nYou can [`read`](crate::Reg::read) this register and get [`pur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pur`] module"]
#[doc(alias = "PUR")]
pub type Pur = crate::Reg<pur::PurSpec>;
#[doc = "desc PUR"]
pub mod pur;
#[doc = "AFRH (rw) register accessor: desc AFRH\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrh`] module"]
#[doc(alias = "AFRH")]
pub type Afrh = crate::Reg<afrh::AfrhSpec>;
#[doc = "desc AFRH"]
pub mod afrh;
#[doc = "AFRL (rw) register accessor: desc AFRL\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`] module"]
#[doc(alias = "AFRL")]
pub type Afrl = crate::Reg<afrl::AfrlSpec>;
#[doc = "desc AFRL"]
pub mod afrl;
#[doc = "ANALOG (rw) register accessor: desc ANALOG\n\nYou can [`read`](crate::Reg::read) this register and get [`analog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog`] module"]
#[doc(alias = "ANALOG")]
pub type Analog = crate::Reg<analog::AnalogSpec>;
#[doc = "desc ANALOG"]
pub mod analog;
#[doc = "DRIVER (rw) register accessor: desc DRIVER\n\nYou can [`read`](crate::Reg::read) this register and get [`driver::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`driver::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@driver`] module"]
#[doc(alias = "DRIVER")]
pub type Driver = crate::Reg<driver::DriverSpec>;
#[doc = "desc DRIVER"]
pub mod driver;
#[doc = "RISEIE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`riseie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`riseie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riseie`] module"]
#[doc(alias = "RISEIE")]
pub type Riseie = crate::Reg<riseie::RiseieSpec>;
#[doc = "Interrupt enable register"]
pub mod riseie;
#[doc = "FALLIE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`fallie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fallie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fallie`] module"]
#[doc(alias = "FALLIE")]
pub type Fallie = crate::Reg<fallie::FallieSpec>;
#[doc = "Interrupt enable register"]
pub mod fallie;
#[doc = "HIGHIE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`highie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`highie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@highie`] module"]
#[doc(alias = "HIGHIE")]
pub type Highie = crate::Reg<highie::HighieSpec>;
#[doc = "Interrupt enable register"]
pub mod highie;
#[doc = "LOWIE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`lowie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowie`] module"]
#[doc(alias = "LOWIE")]
pub type Lowie = crate::Reg<lowie::LowieSpec>;
#[doc = "Interrupt enable register"]
pub mod lowie;
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
#[doc = "LOCK (rw) register accessor: desc LOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "desc LOCK"]
pub mod lock;
#[doc = "FILTER (rw) register accessor: desc FILTER\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter`] module"]
#[doc(alias = "FILTER")]
pub type Filter = crate::Reg<filter::FilterSpec>;
#[doc = "desc FILTER"]
pub mod filter;
#[doc = "IDR (r) register accessor: desc IDR\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "desc IDR"]
pub mod idr;
#[doc = "ODR (rw) register accessor: desc ODR\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`] module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "desc ODR"]
pub mod odr;
#[doc = "ODRLOWBYTE (rw) register accessor: desc ODRLOWBYTE\n\nYou can [`read`](crate::Reg::read) this register and get [`odrlowbyte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odrlowbyte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odrlowbyte`] module"]
#[doc(alias = "ODRLOWBYTE")]
pub type Odrlowbyte = crate::Reg<odrlowbyte::OdrlowbyteSpec>;
#[doc = "desc ODRLOWBYTE"]
pub mod odrlowbyte;
#[doc = "ODRHIGHBYTE (rw) register accessor: desc ODRHIGHBYTE\n\nYou can [`read`](crate::Reg::read) this register and get [`odrhighbyte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odrhighbyte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odrhighbyte`] module"]
#[doc(alias = "ODRHIGHBYTE")]
pub type Odrhighbyte = crate::Reg<odrhighbyte::OdrhighbyteSpec>;
#[doc = "desc ODRHIGHBYTE"]
pub mod odrhighbyte;
#[doc = "BRR (rw) register accessor: desc BRR\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "desc BRR"]
pub mod brr;
#[doc = "BSRR (rw) register accessor: desc BSRR\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`] module"]
#[doc(alias = "BSRR")]
pub type Bsrr = crate::Reg<bsrr::BsrrSpec>;
#[doc = "desc BSRR"]
pub mod bsrr;
#[doc = "TOG (rw) register accessor: desc TOG\n\nYou can [`read`](crate::Reg::read) this register and get [`tog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tog`] module"]
#[doc(alias = "TOG")]
pub type Tog = crate::Reg<tog::TogSpec>;
#[doc = "desc TOG"]
pub mod tog;
