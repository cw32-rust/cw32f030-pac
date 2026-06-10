#[doc = "Register `LSI` reader"]
pub type R = crate::R<LsiSpec>;
#[doc = "Register `LSI` writer"]
pub type W = crate::W<LsiSpec>;
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TrimR = crate::FieldReader<u16>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WaitcycleR = crate::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WaitcycleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type StableR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&self) -> WaitcycleR {
        WaitcycleR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 15 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&mut self) -> TrimW<'_, LsiSpec> {
        TrimW::new(self, 0)
    }
    #[doc = "Bits 10:11 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&mut self) -> WaitcycleW<'_, LsiSpec> {
        WaitcycleW::new(self, 10)
    }
}
#[doc = "LSI Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`lsi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsiSpec;
impl crate::RegisterSpec for LsiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsi::R`](R) reader structure"]
impl crate::Readable for LsiSpec {}
#[doc = "`write(|w| ..)` method takes [`lsi::W`](W) writer structure"]
impl crate::Writable for LsiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LSI to value 0"]
impl crate::Resettable for LsiSpec {}
