#[doc = "Register `DIV` reader"]
pub type R = crate::R<DivSpec>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DivSpec>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIN` reader - desc VIN"]
pub type VinR = crate::BitReader;
#[doc = "Field `VIN` writer - desc VIN"]
pub type VinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc VIN"]
    #[inline(always)]
    pub fn vin(&self) -> VinR {
        VinR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, DivSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, DivSpec> {
        EnW::new(self, 6)
    }
    #[doc = "Bit 7 - desc VIN"]
    #[inline(always)]
    pub fn vin(&mut self) -> VinW<'_, DivSpec> {
        VinW::new(self, 7)
    }
}
#[doc = "desc DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivSpec;
impl crate::RegisterSpec for DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DivSpec {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DivSpec {}
