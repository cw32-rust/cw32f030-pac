#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CrSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
