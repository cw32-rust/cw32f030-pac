#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `ACCESS` reader - desc ACCESS"]
pub type AccessR = crate::BitReader;
#[doc = "Field `ACCESS` writer - desc ACCESS"]
pub type AccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINDOW` reader - desc WINDOW"]
pub type WindowR = crate::BitReader;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SourceR = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - desc ACCESS"]
    #[inline(always)]
    pub fn access(&self) -> AccessR {
        AccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WINDOW"]
    #[inline(always)]
    pub fn window(&self) -> WindowR {
        WindowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc ACCESS"]
    #[inline(always)]
    pub fn access(&mut self) -> AccessW<'_, Cr1Spec> {
        AccessW::new(self, 0)
    }
    #[doc = "Bits 8:10 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, Cr1Spec> {
        SourceW::new(self, 8)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
