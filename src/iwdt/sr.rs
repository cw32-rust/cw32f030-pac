#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `CRF` reader - desc CRF"]
pub type CrfR = crate::BitReader;
#[doc = "Field `ARRF` reader - desc ARRF"]
pub type ArrfR = crate::BitReader;
#[doc = "Field `WINRF` reader - desc WINRF"]
pub type WinrfR = crate::BitReader;
#[doc = "Field `OV` reader - desc OV"]
pub type OvR = crate::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN` reader - desc RUN"]
pub type RunR = crate::BitReader;
#[doc = "Field `RELOAD` reader - desc RELOAD"]
pub type ReloadR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CRF"]
    #[inline(always)]
    pub fn crf(&self) -> CrfR {
        CrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ARRF"]
    #[inline(always)]
    pub fn arrf(&self) -> ArrfR {
        ArrfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc WINRF"]
    #[inline(always)]
    pub fn winrf(&self) -> WinrfR {
        WinrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OvR {
        OvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RUN"]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RELOAD"]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - desc OV"]
    #[inline(always)]
    pub fn ov(&mut self) -> OvW<'_, SrSpec> {
        OvW::new(self, 3)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
