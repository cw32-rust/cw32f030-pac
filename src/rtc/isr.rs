#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `ALARMA` reader - desc ALARMA"]
pub type AlarmaR = crate::BitReader;
#[doc = "Field `ALARMB` reader - desc ALARMB"]
pub type AlarmbR = crate::BitReader;
#[doc = "Field `AWTIMER` reader - desc AWTIMER"]
pub type AwtimerR = crate::BitReader;
#[doc = "Field `TAMP` reader - desc TAMP"]
pub type TampR = crate::BitReader;
#[doc = "Field `TAMPOV` reader - desc TAMPOV"]
pub type TampovR = crate::BitReader;
#[doc = "Field `INTERVAL` reader - desc INTERVAL"]
pub type IntervalR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc ALARMA"]
    #[inline(always)]
    pub fn alarma(&self) -> AlarmaR {
        AlarmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ALARMB"]
    #[inline(always)]
    pub fn alarmb(&self) -> AlarmbR {
        AlarmbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc AWTIMER"]
    #[inline(always)]
    pub fn awtimer(&self) -> AwtimerR {
        AwtimerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TAMP"]
    #[inline(always)]
    pub fn tamp(&self) -> TampR {
        TampR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TAMPOV"]
    #[inline(always)]
    pub fn tampov(&self) -> TampovR {
        TampovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc INTERVAL"]
    #[inline(always)]
    pub fn interval(&self) -> IntervalR {
        IntervalR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
