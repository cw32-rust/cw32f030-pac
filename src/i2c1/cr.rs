#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `FLT` reader - desc FLT"]
pub type FltR = crate::BitReader;
#[doc = "Field `FLT` writer - desc FLT"]
pub type FltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AA` reader - desc AA"]
pub type AaR = crate::BitReader;
#[doc = "Field `AA` writer - desc AA"]
pub type AaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SI` reader - desc SI"]
pub type SiR = crate::BitReader;
#[doc = "Field `SI` writer - desc SI"]
pub type SiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STO` reader - desc STO"]
pub type StoR = crate::BitReader;
#[doc = "Field `STO` writer - desc STO"]
pub type StoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA` reader - desc STA"]
pub type StaR = crate::BitReader;
#[doc = "Field `STA` writer - desc STA"]
pub type StaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc FLT"]
    #[inline(always)]
    pub fn flt(&self) -> FltR {
        FltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc AA"]
    #[inline(always)]
    pub fn aa(&self) -> AaR {
        AaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SI"]
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STO"]
    #[inline(always)]
    pub fn sto(&self) -> StoR {
        StoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc STA"]
    #[inline(always)]
    pub fn sta(&self) -> StaR {
        StaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc FLT"]
    #[inline(always)]
    pub fn flt(&mut self) -> FltW<'_, CrSpec> {
        FltW::new(self, 0)
    }
    #[doc = "Bit 2 - desc AA"]
    #[inline(always)]
    pub fn aa(&mut self) -> AaW<'_, CrSpec> {
        AaW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SI"]
    #[inline(always)]
    pub fn si(&mut self) -> SiW<'_, CrSpec> {
        SiW::new(self, 3)
    }
    #[doc = "Bit 4 - desc STO"]
    #[inline(always)]
    pub fn sto(&mut self) -> StoW<'_, CrSpec> {
        StoW::new(self, 4)
    }
    #[doc = "Bit 5 - desc STA"]
    #[inline(always)]
    pub fn sta(&mut self) -> StaW<'_, CrSpec> {
        StaW::new(self, 5)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 6)
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
