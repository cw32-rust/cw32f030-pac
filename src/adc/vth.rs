#[doc = "Register `VTH` reader"]
pub type R = crate::R<VthSpec>;
#[doc = "Register `VTH` writer"]
pub type W = crate::W<VthSpec>;
#[doc = "Field `VTH` reader - desc VTH"]
pub type VthR = crate::FieldReader<u16>;
#[doc = "Field `VTH` writer - desc VTH"]
pub type VthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - desc VTH"]
    #[inline(always)]
    pub fn vth(&self) -> VthR {
        VthR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc VTH"]
    #[inline(always)]
    pub fn vth(&mut self) -> VthW<'_, VthSpec> {
        VthW::new(self, 0)
    }
}
#[doc = "desc VTH\n\nYou can [`read`](crate::Reg::read) this register and get [`vth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VthSpec;
impl crate::RegisterSpec for VthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vth::R`](R) reader structure"]
impl crate::Readable for VthSpec {}
#[doc = "`write(|w| ..)` method takes [`vth::W`](W) writer structure"]
impl crate::Writable for VthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VTH to value 0"]
impl crate::Resettable for VthSpec {}
