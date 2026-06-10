#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    key: Key,
    cr0: Cr0,
    cr1: Cr1,
    cr2: Cr2,
    compen: Compen,
    date: Date,
    time: Time,
    alarma: Alarma,
    alarmb: Alarmb,
    tampdate: Tampdate,
    tamptime: Tamptime,
    awtarr: Awtarr,
    ier: Ier,
    isr: Isr,
    icr: Icr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc KEY"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x04 - Control register0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x08 - Control register1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x0c - Control register2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x10 - Compen register"]
    #[inline(always)]
    pub const fn compen(&self) -> &Compen {
        &self.compen
    }
    #[doc = "0x14 - Time.Second register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x18 - Time.Second register"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x1c - Alarm - A"]
    #[inline(always)]
    pub const fn alarma(&self) -> &Alarma {
        &self.alarma
    }
    #[doc = "0x20 - Alarm - B"]
    #[inline(always)]
    pub const fn alarmb(&self) -> &Alarmb {
        &self.alarmb
    }
    #[doc = "0x24 - desc TAMPDATE"]
    #[inline(always)]
    pub const fn tampdate(&self) -> &Tampdate {
        &self.tampdate
    }
    #[doc = "0x28 - desc TAMPTIME"]
    #[inline(always)]
    pub const fn tamptime(&self) -> &Tamptime {
        &self.tamptime
    }
    #[doc = "0x2c - Auto Wakeup Timer Auto Reload Register"]
    #[inline(always)]
    pub const fn awtarr(&self) -> &Awtarr {
        &self.awtarr
    }
    #[doc = "0x30 - Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
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
}
#[doc = "KEY (w) register accessor: desc KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "desc KEY"]
pub mod key;
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
#[doc = "CR2 (rw) register accessor: Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control register2"]
pub mod cr2;
#[doc = "COMPEN (rw) register accessor: Compen register\n\nYou can [`read`](crate::Reg::read) this register and get [`compen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compen`] module"]
#[doc(alias = "COMPEN")]
pub type Compen = crate::Reg<compen::CompenSpec>;
#[doc = "Compen register"]
pub mod compen;
#[doc = "DATE (rw) register accessor: Time.Second register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Time.Second register"]
pub mod date;
#[doc = "TIME (rw) register accessor: Time.Second register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`] module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "Time.Second register"]
pub mod time;
#[doc = "ALARMA (rw) register accessor: Alarm - A\n\nYou can [`read`](crate::Reg::read) this register and get [`alarma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarma`] module"]
#[doc(alias = "ALARMA")]
pub type Alarma = crate::Reg<alarma::AlarmaSpec>;
#[doc = "Alarm - A"]
pub mod alarma;
#[doc = "ALARMB (rw) register accessor: Alarm - B\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmb`] module"]
#[doc(alias = "ALARMB")]
pub type Alarmb = crate::Reg<alarmb::AlarmbSpec>;
#[doc = "Alarm - B"]
pub mod alarmb;
#[doc = "TAMPDATE (rw) register accessor: desc TAMPDATE\n\nYou can [`read`](crate::Reg::read) this register and get [`tampdate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampdate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampdate`] module"]
#[doc(alias = "TAMPDATE")]
pub type Tampdate = crate::Reg<tampdate::TampdateSpec>;
#[doc = "desc TAMPDATE"]
pub mod tampdate;
#[doc = "TAMPTIME (rw) register accessor: desc TAMPTIME\n\nYou can [`read`](crate::Reg::read) this register and get [`tamptime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamptime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamptime`] module"]
#[doc(alias = "TAMPTIME")]
pub type Tamptime = crate::Reg<tamptime::TamptimeSpec>;
#[doc = "desc TAMPTIME"]
pub mod tamptime;
#[doc = "AWTARR (rw) register accessor: Auto Wakeup Timer Auto Reload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`awtarr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awtarr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awtarr`] module"]
#[doc(alias = "AWTARR")]
pub type Awtarr = crate::Reg<awtarr::AwtarrSpec>;
#[doc = "Auto Wakeup Timer Auto Reload Register"]
pub mod awtarr;
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
