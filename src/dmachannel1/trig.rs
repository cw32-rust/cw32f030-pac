#[doc = "Register `TRIG` reader"]
pub type R = crate::R<TrigSpec>;
#[doc = "Register `TRIG` writer"]
pub type W = crate::W<TrigSpec>;
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
    pub fn type_(&mut self) -> TypeW<'_, TrigSpec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SOFTSRC"]
    #[inline(always)]
    pub fn softsrc(&mut self) -> SoftsrcW<'_, TrigSpec> {
        SoftsrcW::new(self, 1)
    }
    #[doc = "Bits 2:7 - desc HARDSRC"]
    #[inline(always)]
    pub fn hardsrc(&mut self) -> HardsrcW<'_, TrigSpec> {
        HardsrcW::new(self, 2)
    }
}
#[doc = "Channel.y trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrigSpec;
impl crate::RegisterSpec for TrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trig::R`](R) reader structure"]
impl crate::Readable for TrigSpec {}
#[doc = "`write(|w| ..)` method takes [`trig::W`](W) writer structure"]
impl crate::Writable for TrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIG to value 0"]
impl crate::Resettable for TrigSpec {}
