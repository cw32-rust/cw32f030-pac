#[doc = "Register `WINR` reader"]
pub type R = crate::R<WinrSpec>;
#[doc = "Register `WINR` writer"]
pub type W = crate::W<WinrSpec>;
#[doc = "Field `WINR` reader - desc WINR"]
pub type WinrR = crate::FieldReader<u16>;
#[doc = "Field `WINR` writer - desc WINR"]
pub type WinrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - desc WINR"]
    #[inline(always)]
    pub fn winr(&self) -> WinrR {
        WinrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc WINR"]
    #[inline(always)]
    pub fn winr(&mut self) -> WinrW<'_, WinrSpec> {
        WinrW::new(self, 0)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::Reg::read) this register and get [`winr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinrSpec;
impl crate::RegisterSpec for WinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winr::R`](R) reader structure"]
impl crate::Readable for WinrSpec {}
#[doc = "`write(|w| ..)` method takes [`winr::W`](W) writer structure"]
impl crate::Writable for WinrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WINR to value 0"]
impl crate::Resettable for WinrSpec {}
