#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `HSIRDY` reader - desc HSIRDY"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSIRDY` writer - desc HSIRDY"]
pub type HsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - desc HSERDY"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSERDY` writer - desc HSERDY"]
pub type HserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - desc PLLRDY"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `PLLRDY` writer - desc PLLRDY"]
pub type PllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - desc LSIRDY"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `LSIRDY` writer - desc LSIRDY"]
pub type LsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - desc LSERDY"]
pub type LserdyR = crate::BitReader;
#[doc = "Field `LSERDY` writer - desc LSERDY"]
pub type LserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEFAIL` reader - desc LSEFAIL"]
pub type LsefailR = crate::BitReader;
#[doc = "Field `LSEFAIL` writer - desc LSEFAIL"]
pub type LsefailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEFAIL` reader - desc HSEFAIL"]
pub type HsefailR = crate::BitReader;
#[doc = "Field `HSEFAIL` writer - desc HSEFAIL"]
pub type HsefailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEFAULT` reader - desc LSEFAULT"]
pub type LsefaultR = crate::BitReader;
#[doc = "Field `LSEFAULT` writer - desc LSEFAULT"]
pub type LsefaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEFAULT` reader - desc HSEFAULT"]
pub type HsefaultR = crate::BitReader;
#[doc = "Field `HSEFAULT` writer - desc HSEFAULT"]
pub type HsefaultW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LSEFAIL"]
    #[inline(always)]
    pub fn lsefail(&self) -> LsefailR {
        LsefailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSEFAIL"]
    #[inline(always)]
    pub fn hsefail(&self) -> HsefailR {
        HsefailR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LSEFAULT"]
    #[inline(always)]
    pub fn lsefault(&self) -> LsefaultR {
        LsefaultR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSEFAULT"]
    #[inline(always)]
    pub fn hsefault(&self) -> HsefaultR {
        HsefaultR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&mut self) -> HsirdyW<'_, IcrSpec> {
        HsirdyW::new(self, 0)
    }
    #[doc = "Bit 1 - desc HSERDY"]
    #[inline(always)]
    pub fn hserdy(&mut self) -> HserdyW<'_, IcrSpec> {
        HserdyW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&mut self) -> PllrdyW<'_, IcrSpec> {
        PllrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - desc LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&mut self) -> LsirdyW<'_, IcrSpec> {
        LsirdyW::new(self, 3)
    }
    #[doc = "Bit 4 - desc LSERDY"]
    #[inline(always)]
    pub fn lserdy(&mut self) -> LserdyW<'_, IcrSpec> {
        LserdyW::new(self, 4)
    }
    #[doc = "Bit 5 - desc LSEFAIL"]
    #[inline(always)]
    pub fn lsefail(&mut self) -> LsefailW<'_, IcrSpec> {
        LsefailW::new(self, 5)
    }
    #[doc = "Bit 6 - desc HSEFAIL"]
    #[inline(always)]
    pub fn hsefail(&mut self) -> HsefailW<'_, IcrSpec> {
        HsefailW::new(self, 6)
    }
    #[doc = "Bit 7 - desc LSEFAULT"]
    #[inline(always)]
    pub fn lsefault(&mut self) -> LsefaultW<'_, IcrSpec> {
        LsefaultW::new(self, 7)
    }
    #[doc = "Bit 8 - desc HSEFAULT"]
    #[inline(always)]
    pub fn hsefault(&mut self) -> HsefaultW<'_, IcrSpec> {
        HsefaultW::new(self, 8)
    }
}
#[doc = "Interupt Clear Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
