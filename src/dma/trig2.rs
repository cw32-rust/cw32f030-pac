#[doc = "Register `TRIG2` reader"]
pub type R = crate::R<Trig2Spec>;
#[doc = "Register `TRIG2` writer"]
pub type W = crate::W<Trig2Spec>;
#[doc = "Field `TYPE` reader - desc TYPE"]
pub type TypeR = crate::BitReader;
#[doc = "Field `TYPE` writer - desc TYPE"]
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTSRC` reader - desc SOFTSRC"]
pub type SoftsrcR = crate::BitReader;
#[doc = "Field `SOFTSRC` writer - desc SOFTSRC"]
pub type SoftsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDSRC` reader - desc HARDSRC"]
pub type HardsrcR = crate::FieldReader;
#[doc = "Field `HARDSRC` writer - desc HARDSRC"]
pub type HardsrcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - desc TYPE"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SOFTSRC"]
    #[inline(always)]
    pub fn softsrc(&self) -> SoftsrcR {
        SoftsrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - desc HARDSRC"]
    #[inline(always)]
    pub fn hardsrc(&self) -> HardsrcR {
        HardsrcR::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc TYPE"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<'_, Trig2Spec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SOFTSRC"]
    #[inline(always)]
    pub fn softsrc(&mut self) -> SoftsrcW<'_, Trig2Spec> {
        SoftsrcW::new(self, 1)
    }
    #[doc = "Bits 2:7 - desc HARDSRC"]
    #[inline(always)]
    pub fn hardsrc(&mut self) -> HardsrcW<'_, Trig2Spec> {
        HardsrcW::new(self, 2)
    }
}
#[doc = "Channel2 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Trig2Spec;
impl crate::RegisterSpec for Trig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trig2::R`](R) reader structure"]
impl crate::Readable for Trig2Spec {}
#[doc = "`write(|w| ..)` method takes [`trig2::W`](W) writer structure"]
impl crate::Writable for Trig2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIG2 to value 0"]
impl crate::Resettable for Trig2Spec {}
