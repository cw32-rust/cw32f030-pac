#[doc = "Register `CMMR` reader"]
pub type R = crate::R<CmmrSpec>;
#[doc = "Register `CMMR` writer"]
pub type W = crate::W<CmmrSpec>;
#[doc = "Field `CC1M` reader - desc CC1M"]
pub type Cc1mR = crate::FieldReader;
#[doc = "Field `CC1M` writer - desc CC1M"]
pub type Cc1mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC2M` reader - desc CC2M"]
pub type Cc2mR = crate::FieldReader;
#[doc = "Field `CC2M` writer - desc CC2M"]
pub type Cc2mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC3M` reader - desc CC3M"]
pub type Cc3mR = crate::FieldReader;
#[doc = "Field `CC3M` writer - desc CC3M"]
pub type Cc3mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC4M` reader - desc CC4M"]
pub type Cc4mR = crate::FieldReader;
#[doc = "Field `CC4M` writer - desc CC4M"]
pub type Cc4mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc CC1M"]
    #[inline(always)]
    pub fn cc1m(&self) -> Cc1mR {
        Cc1mR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc CC2M"]
    #[inline(always)]
    pub fn cc2m(&self) -> Cc2mR {
        Cc2mR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc CC3M"]
    #[inline(always)]
    pub fn cc3m(&self) -> Cc3mR {
        Cc3mR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc CC4M"]
    #[inline(always)]
    pub fn cc4m(&self) -> Cc4mR {
        Cc4mR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CC1M"]
    #[inline(always)]
    pub fn cc1m(&mut self) -> Cc1mW<'_, CmmrSpec> {
        Cc1mW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc CC2M"]
    #[inline(always)]
    pub fn cc2m(&mut self) -> Cc2mW<'_, CmmrSpec> {
        Cc2mW::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc CC3M"]
    #[inline(always)]
    pub fn cc3m(&mut self) -> Cc3mW<'_, CmmrSpec> {
        Cc3mW::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc CC4M"]
    #[inline(always)]
    pub fn cc4m(&mut self) -> Cc4mW<'_, CmmrSpec> {
        Cc4mW::new(self, 12)
    }
}
#[doc = "Capture compare control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmmrSpec;
impl crate::RegisterSpec for CmmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmmr::R`](R) reader structure"]
impl crate::Readable for CmmrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmmr::W`](W) writer structure"]
impl crate::Writable for CmmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMMR to value 0"]
impl crate::Resettable for CmmrSpec {}
