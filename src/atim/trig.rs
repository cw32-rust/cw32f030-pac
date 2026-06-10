#[doc = "Register `TRIG` reader"]
pub type R = crate::R<TrigSpec>;
#[doc = "Register `TRIG` writer"]
pub type W = crate::W<TrigSpec>;
#[doc = "Field `UEVE` reader - desc UEVE"]
pub type UeveR = crate::BitReader;
#[doc = "Field `UEVE` writer - desc UEVE"]
pub type UeveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM1AE` reader - desc CM1AE"]
pub type Cm1aeR = crate::BitReader;
#[doc = "Field `CM1AE` writer - desc CM1AE"]
pub type Cm1aeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM2AE` reader - desc CM2AE"]
pub type Cm2aeR = crate::BitReader;
#[doc = "Field `CM2AE` writer - desc CM2AE"]
pub type Cm2aeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM3AE` reader - desc CM3AE"]
pub type Cm3aeR = crate::BitReader;
#[doc = "Field `CM3AE` writer - desc CM3AE"]
pub type Cm3aeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM1BE` reader - desc CM1BE"]
pub type Cm1beR = crate::BitReader;
#[doc = "Field `CM1BE` writer - desc CM1BE"]
pub type Cm1beW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM2BE` reader - desc CM2BE"]
pub type Cm2beR = crate::BitReader;
#[doc = "Field `CM2BE` writer - desc CM2BE"]
pub type Cm2beW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM3BE` reader - desc CM3BE"]
pub type Cm3beR = crate::BitReader;
#[doc = "Field `CM3BE` writer - desc CM3BE"]
pub type Cm3beW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTE` reader - desc ADTE"]
pub type AdteR = crate::BitReader;
#[doc = "Field `ADTE` writer - desc ADTE"]
pub type AdteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UEVE"]
    #[inline(always)]
    pub fn ueve(&self) -> UeveR {
        UeveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CM1AE"]
    #[inline(always)]
    pub fn cm1ae(&self) -> Cm1aeR {
        Cm1aeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CM2AE"]
    #[inline(always)]
    pub fn cm2ae(&self) -> Cm2aeR {
        Cm2aeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CM3AE"]
    #[inline(always)]
    pub fn cm3ae(&self) -> Cm3aeR {
        Cm3aeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CM1BE"]
    #[inline(always)]
    pub fn cm1be(&self) -> Cm1beR {
        Cm1beR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CM2BE"]
    #[inline(always)]
    pub fn cm2be(&self) -> Cm2beR {
        Cm2beR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CM3BE"]
    #[inline(always)]
    pub fn cm3be(&self) -> Cm3beR {
        Cm3beR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ADTE"]
    #[inline(always)]
    pub fn adte(&self) -> AdteR {
        AdteR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UEVE"]
    #[inline(always)]
    pub fn ueve(&mut self) -> UeveW<'_, TrigSpec> {
        UeveW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CM1AE"]
    #[inline(always)]
    pub fn cm1ae(&mut self) -> Cm1aeW<'_, TrigSpec> {
        Cm1aeW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CM2AE"]
    #[inline(always)]
    pub fn cm2ae(&mut self) -> Cm2aeW<'_, TrigSpec> {
        Cm2aeW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CM3AE"]
    #[inline(always)]
    pub fn cm3ae(&mut self) -> Cm3aeW<'_, TrigSpec> {
        Cm3aeW::new(self, 3)
    }
    #[doc = "Bit 4 - desc CM1BE"]
    #[inline(always)]
    pub fn cm1be(&mut self) -> Cm1beW<'_, TrigSpec> {
        Cm1beW::new(self, 4)
    }
    #[doc = "Bit 5 - desc CM2BE"]
    #[inline(always)]
    pub fn cm2be(&mut self) -> Cm2beW<'_, TrigSpec> {
        Cm2beW::new(self, 5)
    }
    #[doc = "Bit 6 - desc CM3BE"]
    #[inline(always)]
    pub fn cm3be(&mut self) -> Cm3beW<'_, TrigSpec> {
        Cm3beW::new(self, 6)
    }
    #[doc = "Bit 7 - desc ADTE"]
    #[inline(always)]
    pub fn adte(&mut self) -> AdteW<'_, TrigSpec> {
        AdteW::new(self, 7)
    }
}
#[doc = "desc TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrigSpec;
impl crate::RegisterSpec for TrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trig::R`](R) reader structure"]
impl crate::Readable for TrigSpec {}
#[doc = "`write(|w| ..)` method takes [`trig::W`](W) writer structure"]
impl crate::Writable for TrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIG to value 0"]
impl crate::Resettable for TrigSpec {}
