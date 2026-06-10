#[doc = "Register `ETR` reader"]
pub type R = crate::R<EtrSpec>;
#[doc = "Register `ETR` writer"]
pub type W = crate::W<EtrSpec>;
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
    pub fn etrflt(&mut self) -> EtrfltW<'_, EtrSpec> {
        EtrfltW::new(self, 4)
    }
}
#[doc = "ETR Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`etr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtrSpec;
impl crate::RegisterSpec for EtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etr::R`](R) reader structure"]
impl crate::Readable for EtrSpec {}
#[doc = "`write(|w| ..)` method takes [`etr::W`](W) writer structure"]
impl crate::Writable for EtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETR to value 0"]
impl crate::Resettable for EtrSpec {}
