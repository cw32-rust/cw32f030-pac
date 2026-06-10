#[doc = "Register `VTL` reader"]
pub type R = crate::R<VtlSpec>;
#[doc = "Register `VTL` writer"]
pub type W = crate::W<VtlSpec>;
#[doc = "Field `VTL` reader - desc VTL"]
pub type VtlR = crate::FieldReader<u16>;
#[doc = "Field `VTL` writer - desc VTL"]
pub type VtlW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - desc VTL"]
    #[inline(always)]
    pub fn vtl(&self) -> VtlR {
        VtlR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc VTL"]
    #[inline(always)]
    pub fn vtl(&mut self) -> VtlW<'_, VtlSpec> {
        VtlW::new(self, 0)
    }
}
#[doc = "desc VTL\n\nYou can [`read`](crate::Reg::read) this register and get [`vtl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vtl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VtlSpec;
impl crate::RegisterSpec for VtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtl::R`](R) reader structure"]
impl crate::Readable for VtlSpec {}
#[doc = "`write(|w| ..)` method takes [`vtl::W`](W) writer structure"]
impl crate::Writable for VtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VTL to value 0"]
impl crate::Resettable for VtlSpec {}
