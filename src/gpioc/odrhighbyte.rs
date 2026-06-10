#[doc = "Register `ODRHIGHBYTE` reader"]
pub type R = crate::R<OdrhighbyteSpec>;
#[doc = "Register `ODRHIGHBYTE` writer"]
pub type W = crate::W<OdrhighbyteSpec>;
#[doc = "Field `HIGHBYTE` reader - desc HIGHBYTE"]
pub type HighbyteR = crate::FieldReader;
#[doc = "Field `HIGHBYTE` writer - desc HIGHBYTE"]
pub type HighbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc HIGHBYTE"]
    #[inline(always)]
    pub fn highbyte(&self) -> HighbyteR {
        HighbyteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc HIGHBYTE"]
    #[inline(always)]
    pub fn highbyte(&mut self) -> HighbyteW<'_, OdrhighbyteSpec> {
        HighbyteW::new(self, 0)
    }
}
#[doc = "desc ODRHIGHBYTE\n\nYou can [`read`](crate::Reg::read) this register and get [`odrhighbyte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odrhighbyte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrhighbyteSpec;
impl crate::RegisterSpec for OdrhighbyteSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`odrhighbyte::R`](R) reader structure"]
impl crate::Readable for OdrhighbyteSpec {}
#[doc = "`write(|w| ..)` method takes [`odrhighbyte::W`](W) writer structure"]
impl crate::Writable for OdrhighbyteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ODRHIGHBYTE to value 0"]
impl crate::Resettable for OdrhighbyteSpec {}
