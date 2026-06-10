#[doc = "Register `HSI` reader"]
pub type R = crate::R<HsiSpec>;
#[doc = "Register `HSI` writer"]
pub type W = crate::W<HsiSpec>;
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TrimR = crate::FieldReader<u16>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type StableR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&mut self) -> TrimW<'_, HsiSpec> {
        TrimW::new(self, 0)
    }
    #[doc = "Bits 11:14 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, HsiSpec> {
        DivW::new(self, 11)
    }
}
#[doc = "HSI Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hsi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsiSpec;
impl crate::RegisterSpec for HsiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsi::R`](R) reader structure"]
impl crate::Readable for HsiSpec {}
#[doc = "`write(|w| ..)` method takes [`hsi::W`](W) writer structure"]
impl crate::Writable for HsiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSI to value 0"]
impl crate::Resettable for HsiSpec {}
