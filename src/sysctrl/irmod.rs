#[doc = "Register `IRMOD` reader"]
pub type R = crate::R<IrmodSpec>;
#[doc = "Register `IRMOD` writer"]
pub type W = crate::W<IrmodSpec>;
#[doc = "Field `MOD` reader - desc MOD"]
pub type ModR = crate::FieldReader;
#[doc = "Field `MOD` writer - desc MOD"]
pub type ModW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc MOD"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MOD"]
    #[inline(always)]
    pub fn mod_(&mut self) -> ModW<'_, IrmodSpec> {
        ModW::new(self, 0)
    }
}
#[doc = "IR MOD Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`irmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrmodSpec;
impl crate::RegisterSpec for IrmodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irmod::R`](R) reader structure"]
impl crate::Readable for IrmodSpec {}
#[doc = "`write(|w| ..)` method takes [`irmod::W`](W) writer structure"]
impl crate::Writable for IrmodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRMOD to value 0"]
impl crate::Resettable for IrmodSpec {}
