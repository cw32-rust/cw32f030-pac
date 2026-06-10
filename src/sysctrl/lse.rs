#[doc = "Register `LSE` reader"]
pub type R = crate::R<LseSpec>;
#[doc = "Register `LSE` writer"]
pub type W = crate::W<LseSpec>;
#[doc = "Field `DRIVER` reader - desc DRIVER"]
pub type DriverR = crate::FieldReader;
#[doc = "Field `DRIVER` writer - desc DRIVER"]
pub type DriverW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMP` reader - desc AMP"]
pub type AmpR = crate::FieldReader;
#[doc = "Field `AMP` writer - desc AMP"]
pub type AmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WaitcycleR = crate::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WaitcycleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type StableR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    pub fn driver(&self) -> DriverR {
        DriverR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc AMP"]
    #[inline(always)]
    pub fn amp(&self) -> AmpR {
        AmpR::new(((self.bits >> 2) & 3) as u8)
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
    #[doc = "Bit 15 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    pub fn driver(&mut self) -> DriverW<'_, LseSpec> {
        DriverW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc AMP"]
    #[inline(always)]
    pub fn amp(&mut self) -> AmpW<'_, LseSpec> {
        AmpW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&mut self) -> WaitcycleW<'_, LseSpec> {
        WaitcycleW::new(self, 4)
    }
    #[doc = "Bit 6 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, LseSpec> {
        ModeW::new(self, 6)
    }
}
#[doc = "LSE Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`lse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LseSpec;
impl crate::RegisterSpec for LseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lse::R`](R) reader structure"]
impl crate::Readable for LseSpec {}
#[doc = "`write(|w| ..)` method takes [`lse::W`](W) writer structure"]
impl crate::Writable for LseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LSE to value 0"]
impl crate::Resettable for LseSpec {}
