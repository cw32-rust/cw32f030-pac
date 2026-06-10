#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `ALARMA` reader - desc ALARMA"]
pub type AlarmaR = crate::BitReader;
#[doc = "Field `ALARMA` writer - desc ALARMA"]
pub type AlarmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARMB` reader - desc ALARMB"]
pub type AlarmbR = crate::BitReader;
#[doc = "Field `ALARMB` writer - desc ALARMB"]
pub type AlarmbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWTIMER` reader - desc AWTIMER"]
pub type AwtimerR = crate::BitReader;
#[doc = "Field `AWTIMER` writer - desc AWTIMER"]
pub type AwtimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP` reader - desc TAMP"]
pub type TampR = crate::BitReader;
#[doc = "Field `TAMP` writer - desc TAMP"]
pub type TampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPOV` reader - desc TAMPOV"]
pub type TampovR = crate::BitReader;
#[doc = "Field `TAMPOV` writer - desc TAMPOV"]
pub type TampovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERVAL` reader - desc INTERVAL"]
pub type IntervalR = crate::BitReader;
#[doc = "Field `INTERVAL` writer - desc INTERVAL"]
pub type IntervalW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - desc ALARMA"]
    #[inline(always)]
    pub fn alarma(&mut self) -> AlarmaW<'_, IcrSpec> {
        AlarmaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc ALARMB"]
    #[inline(always)]
    pub fn alarmb(&mut self) -> AlarmbW<'_, IcrSpec> {
        AlarmbW::new(self, 1)
    }
    #[doc = "Bit 2 - desc AWTIMER"]
    #[inline(always)]
    pub fn awtimer(&mut self) -> AwtimerW<'_, IcrSpec> {
        AwtimerW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TAMP"]
    #[inline(always)]
    pub fn tamp(&mut self) -> TampW<'_, IcrSpec> {
        TampW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TAMPOV"]
    #[inline(always)]
    pub fn tampov(&mut self) -> TampovW<'_, IcrSpec> {
        TampovW::new(self, 4)
    }
    #[doc = "Bit 6 - desc INTERVAL"]
    #[inline(always)]
    pub fn interval(&mut self) -> IntervalW<'_, IcrSpec> {
        IntervalW::new(self, 6)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
