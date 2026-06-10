#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `WAIT` reader - desc WAIT"]
pub type WaitR = crate::FieldReader;
#[doc = "Field `WAIT` writer - desc WAIT"]
pub type WaitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FETCH` reader - desc FETCH"]
pub type FetchR = crate::BitReader;
#[doc = "Field `FETCH` writer - desc FETCH"]
pub type FetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE` reader - desc CACHE"]
pub type CacheR = crate::BitReader;
#[doc = "Field `CACHE` writer - desc CACHE"]
pub type CacheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc FETCH"]
    #[inline(always)]
    pub fn fetch(&self) -> FetchR {
        FetchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CACHE"]
    #[inline(always)]
    pub fn cache(&self) -> CacheR {
        CacheR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<'_, Cr2Spec> {
        WaitW::new(self, 0)
    }
    #[doc = "Bit 3 - desc FETCH"]
    #[inline(always)]
    pub fn fetch(&mut self) -> FetchW<'_, Cr2Spec> {
        FetchW::new(self, 3)
    }
    #[doc = "Bit 4 - desc CACHE"]
    #[inline(always)]
    pub fn cache(&mut self) -> CacheW<'_, Cr2Spec> {
        CacheW::new(self, 4)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Cr2Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
