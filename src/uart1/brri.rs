#[doc = "Register `BRRI` reader"]
pub type R = crate::R<BrriSpec>;
#[doc = "Register `BRRI` writer"]
pub type W = crate::W<BrriSpec>;
#[doc = "Field `BRRI` reader - desc BRRI"]
pub type BrriR = crate::FieldReader<u16>;
#[doc = "Field `BRRI` writer - desc BRRI"]
pub type BrriW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc BRRI"]
    #[inline(always)]
    pub fn brri(&self) -> BrriR {
        BrriR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc BRRI"]
    #[inline(always)]
    pub fn brri(&mut self) -> BrriW<'_, BrriSpec> {
        BrriW::new(self, 0)
    }
}
#[doc = "desc BRRI\n\nYou can [`read`](crate::Reg::read) this register and get [`brri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrriSpec;
impl crate::RegisterSpec for BrriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brri::R`](R) reader structure"]
impl crate::Readable for BrriSpec {}
#[doc = "`write(|w| ..)` method takes [`brri::W`](W) writer structure"]
impl crate::Writable for BrriSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRRI to value 0"]
impl crate::Resettable for BrriSpec {}
