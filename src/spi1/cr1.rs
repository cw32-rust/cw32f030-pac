#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CPHA` reader - desc CPHA"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - desc CPHA"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - desc CPOL"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - desc CPOL"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTR` reader - desc MSTR"]
pub type MstrR = crate::BitReader;
#[doc = "Field `MSTR` writer - desc MSTR"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR` reader - desc BR"]
pub type BrR = crate::FieldReader;
#[doc = "Field `BR` writer - desc BR"]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBF` reader - desc LSBF"]
pub type LsbfR = crate::BitReader;
#[doc = "Field `LSBF` writer - desc LSBF"]
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMP` reader - desc SMP"]
pub type SmpR = crate::BitReader;
#[doc = "Field `SMP` writer - desc SMP"]
pub type SmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - desc SSM"]
pub type SsmR = crate::BitReader;
#[doc = "Field `SSM` writer - desc SSM"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIDTH` reader - desc WIDTH"]
pub type WidthR = crate::FieldReader;
#[doc = "Field `WIDTH` writer - desc WIDTH"]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMARX` reader - desc DMARX"]
pub type DmarxR = crate::BitReader;
#[doc = "Field `DMARX` writer - desc DMARX"]
pub type DmarxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATX` reader - desc DMATX"]
pub type DmatxR = crate::BitReader;
#[doc = "Field `DMATX` writer - desc DMATX"]
pub type DmatxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISOHD` reader - desc MISOHD"]
pub type MisohdR = crate::BitReader;
#[doc = "Field `MISOHD` writer - desc MISOHD"]
pub type MisohdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - desc BR"]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LSBF"]
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SMP"]
    #[inline(always)]
    pub fn smp(&self) -> SmpR {
        SmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SSM"]
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - desc WIDTH"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - desc DMARX"]
    #[inline(always)]
    pub fn dmarx(&self) -> DmarxR {
        DmarxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc DMATX"]
    #[inline(always)]
    pub fn dmatx(&self) -> DmatxR {
        DmatxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc MISOHD"]
    #[inline(always)]
    pub fn misohd(&self) -> MisohdR {
        MisohdR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Cr1Spec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Cr1Spec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bit 2 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<'_, Cr1Spec> {
        MstrW::new(self, 2)
    }
    #[doc = "Bits 3:5 - desc BR"]
    #[inline(always)]
    pub fn br(&mut self) -> BrW<'_, Cr1Spec> {
        BrW::new(self, 3)
    }
    #[doc = "Bit 6 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr1Spec> {
        EnW::new(self, 6)
    }
    #[doc = "Bit 7 - desc LSBF"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LsbfW<'_, Cr1Spec> {
        LsbfW::new(self, 7)
    }
    #[doc = "Bit 8 - desc SMP"]
    #[inline(always)]
    pub fn smp(&mut self) -> SmpW<'_, Cr1Spec> {
        SmpW::new(self, 8)
    }
    #[doc = "Bit 9 - desc SSM"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<'_, Cr1Spec> {
        SsmW::new(self, 9)
    }
    #[doc = "Bits 10:13 - desc WIDTH"]
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<'_, Cr1Spec> {
        WidthW::new(self, 10)
    }
    #[doc = "Bits 14:15 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cr1Spec> {
        ModeW::new(self, 14)
    }
    #[doc = "Bit 16 - desc DMARX"]
    #[inline(always)]
    pub fn dmarx(&mut self) -> DmarxW<'_, Cr1Spec> {
        DmarxW::new(self, 16)
    }
    #[doc = "Bit 17 - desc DMATX"]
    #[inline(always)]
    pub fn dmatx(&mut self) -> DmatxW<'_, Cr1Spec> {
        DmatxW::new(self, 17)
    }
    #[doc = "Bit 18 - desc MISOHD"]
    #[inline(always)]
    pub fn misohd(&mut self) -> MisohdW<'_, Cr1Spec> {
        MisohdW::new(self, 18)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
