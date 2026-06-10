#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `ETRFLT` reader - desc ETRFLT"]
pub type EtrfltR = crate::FieldReader;
#[doc = "Field `ETRFLT` writer - desc ETRFLT"]
pub type EtrfltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:6 - desc ETRFLT"]
    #[inline(always)]
    pub fn etrflt(&self) -> EtrfltR {
        EtrfltR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - desc ETRFLT"]
    #[inline(always)]
    pub fn etrflt(&mut self) -> EtrfltW<'_, AcrSpec> {
        EtrfltW::new(self, 4)
    }
}
#[doc = "Advanced Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for AcrSpec {}
