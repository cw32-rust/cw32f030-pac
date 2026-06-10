#[doc = "Register `HSE` reader"]
pub type R = crate::R<HseSpec>;
#[doc = "Register `HSE` writer"]
pub type W = crate::W<HseSpec>;
#[doc = "Field `DRIVER` reader - desc DRIVER"]
pub type DriverR = crate::FieldReader;
#[doc = "Field `DRIVER` writer - desc DRIVER"]
pub type DriverW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREQRANGE` reader - desc FREQRANGE"]
pub type FreqrangeR = crate::FieldReader;
#[doc = "Field `FREQRANGE` writer - desc FREQRANGE"]
pub type FreqrangeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WaitcycleR = crate::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WaitcycleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT` reader - desc FLT"]
pub type FltR = crate::BitReader;
#[doc = "Field `FLT` writer - desc FLT"]
pub type FltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETCNT` reader - desc DETCNT"]
pub type DetcntR = crate::FieldReader<u16>;
#[doc = "Field `DETCNT` writer - desc DETCNT"]
pub type DetcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type StableR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    pub fn driver(&self) -> DriverR {
        DriverR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc FREQRANGE"]
    #[inline(always)]
    pub fn freqrange(&self) -> FreqrangeR {
        FreqrangeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&self) -> WaitcycleR {
        WaitcycleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc FLT"]
    #[inline(always)]
    pub fn flt(&self) -> FltR {
        FltR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:18 - desc DETCNT"]
    #[inline(always)]
    pub fn detcnt(&self) -> DetcntR {
        DetcntR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bit 19 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    pub fn driver(&mut self) -> DriverW<'_, HseSpec> {
        DriverW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc FREQRANGE"]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FreqrangeW<'_, HseSpec> {
        FreqrangeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&mut self) -> WaitcycleW<'_, HseSpec> {
        WaitcycleW::new(self, 4)
    }
    #[doc = "Bit 6 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, HseSpec> {
        ModeW::new(self, 6)
    }
    #[doc = "Bit 7 - desc FLT"]
    #[inline(always)]
    pub fn flt(&mut self) -> FltW<'_, HseSpec> {
        FltW::new(self, 7)
    }
    #[doc = "Bits 8:18 - desc DETCNT"]
    #[inline(always)]
    pub fn detcnt(&mut self) -> DetcntW<'_, HseSpec> {
        DetcntW::new(self, 8)
    }
}
#[doc = "HSE Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HseSpec;
impl crate::RegisterSpec for HseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hse::R`](R) reader structure"]
impl crate::Readable for HseSpec {}
#[doc = "`write(|w| ..)` method takes [`hse::W`](W) writer structure"]
impl crate::Writable for HseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSE to value 0"]
impl crate::Resettable for HseSpec {}
