#[doc = "Register `ODRLOWBYTE` reader"]
pub type R = crate::R<OdrlowbyteSpec>;
#[doc = "Register `ODRLOWBYTE` writer"]
pub type W = crate::W<OdrlowbyteSpec>;
#[doc = "Field `LOWBYTE` reader - desc LOWBYTE"]
pub type LowbyteR = crate::FieldReader;
#[doc = "Field `LOWBYTE` writer - desc LOWBYTE"]
pub type LowbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc LOWBYTE"]
    #[inline(always)]
    pub fn lowbyte(&self) -> LowbyteR {
        LowbyteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc LOWBYTE"]
    #[inline(always)]
    pub fn lowbyte(&mut self) -> LowbyteW<'_, OdrlowbyteSpec> {
        LowbyteW::new(self, 0)
    }
}
#[doc = "desc ODRLOWBYTE\n\nYou can [`read`](crate::Reg::read) this register and get [`odrlowbyte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odrlowbyte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrlowbyteSpec;
impl crate::RegisterSpec for OdrlowbyteSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`odrlowbyte::R`](R) reader structure"]
impl crate::Readable for OdrlowbyteSpec {}
#[doc = "`write(|w| ..)` method takes [`odrlowbyte::W`](W) writer structure"]
impl crate::Writable for OdrlowbyteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ODRLOWBYTE to value 0"]
impl crate::Resettable for OdrlowbyteSpec {}
