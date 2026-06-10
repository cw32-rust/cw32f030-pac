#[doc = "Register `SSI` reader"]
pub type R = crate::R<SsiSpec>;
#[doc = "Register `SSI` writer"]
pub type W = crate::W<SsiSpec>;
#[doc = "Field `SSI` reader - desc SSI"]
pub type SsiR = crate::BitReader;
#[doc = "Field `SSI` writer - desc SSI"]
pub type SsiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SSI"]
    #[inline(always)]
    pub fn ssi(&self) -> SsiR {
        SsiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SSI"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SsiW<'_, SsiSpec> {
        SsiW::new(self, 0)
    }
}
#[doc = "Slave slect register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsiSpec;
impl crate::RegisterSpec for SsiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssi::R`](R) reader structure"]
impl crate::Readable for SsiSpec {}
#[doc = "`write(|w| ..)` method takes [`ssi::W`](W) writer structure"]
impl crate::Writable for SsiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSI to value 0"]
impl crate::Resettable for SsiSpec {}
