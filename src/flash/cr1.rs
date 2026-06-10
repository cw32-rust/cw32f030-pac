#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STANDBY` reader - desc STANDBY"]
pub type StandbyR = crate::BitReader;
#[doc = "Field `STANDBY` writer - desc STANDBY"]
pub type StandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `SECURITY` reader - desc SECURITY"]
pub type SecurityR = crate::FieldReader;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - desc STANDBY"]
    #[inline(always)]
    pub fn standby(&self) -> StandbyR {
        StandbyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc SECURITY"]
    #[inline(always)]
    pub fn security(&self) -> SecurityR {
        SecurityR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cr1Spec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - desc STANDBY"]
    #[inline(always)]
    pub fn standby(&mut self) -> StandbyW<'_, Cr1Spec> {
        StandbyW::new(self, 4)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Cr1Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
