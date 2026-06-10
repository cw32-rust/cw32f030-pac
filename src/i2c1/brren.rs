#[doc = "Register `BRREN` reader"]
pub type R = crate::R<BrrenSpec>;
#[doc = "Register `BRREN` writer"]
pub type W = crate::W<BrrenSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, BrrenSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "desc BRREN\n\nYou can [`read`](crate::Reg::read) this register and get [`brren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrenSpec;
impl crate::RegisterSpec for BrrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brren::R`](R) reader structure"]
impl crate::Readable for BrrenSpec {}
#[doc = "`write(|w| ..)` method takes [`brren::W`](W) writer structure"]
impl crate::Writable for BrrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRREN to value 0"]
impl crate::Resettable for BrrenSpec {}
