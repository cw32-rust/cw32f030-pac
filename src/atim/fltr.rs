#[doc = "Register `FLTR` reader"]
pub type R = crate::R<FltrSpec>;
#[doc = "Register `FLTR` writer"]
pub type W = crate::W<FltrSpec>;
#[doc = "Field `OCM1AFLT1A` reader - desc OCM1AFLT1A"]
pub type Ocm1aflt1aR = crate::FieldReader;
#[doc = "Field `OCM1AFLT1A` writer - desc OCM1AFLT1A"]
pub type Ocm1aflt1aW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCP1A` reader - desc CCP1A"]
pub type Ccp1aR = crate::BitReader;
#[doc = "Field `CCP1A` writer - desc CCP1A"]
pub type Ccp1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCM1BFLT1B` reader - desc OCM1BFLT1B"]
pub type Ocm1bflt1bR = crate::FieldReader;
#[doc = "Field `OCM1BFLT1B` writer - desc OCM1BFLT1B"]
pub type Ocm1bflt1bW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCP1B` reader - desc CCP1B"]
pub type Ccp1bR = crate::BitReader;
#[doc = "Field `CCP1B` writer - desc CCP1B"]
pub type Ccp1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCM2AFLT2A` reader - desc OCM2AFLT2A"]
pub type Ocm2aflt2aR = crate::FieldReader;
#[doc = "Field `OCM2AFLT2A` writer - desc OCM2AFLT2A"]
pub type Ocm2aflt2aW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCP2A` reader - desc CCP2A"]
pub type Ccp2aR = crate::BitReader;
#[doc = "Field `CCP2A` writer - desc CCP2A"]
pub type Ccp2aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCM2BFLT2B` reader - desc OCM2BFLT2B"]
pub type Ocm2bflt2bR = crate::FieldReader;
#[doc = "Field `OCM2BFLT2B` writer - desc OCM2BFLT2B"]
pub type Ocm2bflt2bW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCP2B` reader - desc CCP2B"]
pub type Ccp2bR = crate::BitReader;
#[doc = "Field `CCP2B` writer - desc CCP2B"]
pub type Ccp2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCM3AFLT3A` reader - desc OCM3AFLT3A"]
pub type Ocm3aflt3aR = crate::FieldReader;
#[doc = "Field `OCM3AFLT3A` writer - desc OCM3AFLT3A"]
pub type Ocm3aflt3aW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCP3A` reader - desc CCP3A"]
pub type Ccp3aR = crate::BitReader;
#[doc = "Field `CCP3A` writer - desc CCP3A"]
pub type Ccp3aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCM3BFLT3B` reader - desc OCM3BFLT3B"]
pub type Ocm3bflt3bR = crate::FieldReader;
#[doc = "Field `OCM3BFLT3B` writer - desc OCM3BFLT3B"]
pub type Ocm3bflt3bW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCP3B` reader - desc CCP3B"]
pub type Ccp3bR = crate::BitReader;
#[doc = "Field `CCP3B` writer - desc CCP3B"]
pub type Ccp3bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTBK` reader - desc FLTBK"]
pub type FltbkR = crate::FieldReader;
#[doc = "Field `FLTBK` writer - desc FLTBK"]
pub type FltbkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BKP` reader - desc BKP"]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - desc BKP"]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTET` reader - desc FLTET"]
pub type FltetR = crate::FieldReader;
#[doc = "Field `FLTET` writer - desc FLTET"]
pub type FltetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETP` reader - desc ETP"]
pub type EtpR = crate::BitReader;
#[doc = "Field `ETP` writer - desc ETP"]
pub type EtpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc OCM1AFLT1A"]
    #[inline(always)]
    pub fn ocm1aflt1a(&self) -> Ocm1aflt1aR {
        Ocm1aflt1aR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc CCP1A"]
    #[inline(always)]
    pub fn ccp1a(&self) -> Ccp1aR {
        Ccp1aR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc OCM1BFLT1B"]
    #[inline(always)]
    pub fn ocm1bflt1b(&self) -> Ocm1bflt1bR {
        Ocm1bflt1bR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc CCP1B"]
    #[inline(always)]
    pub fn ccp1b(&self) -> Ccp1bR {
        Ccp1bR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc OCM2AFLT2A"]
    #[inline(always)]
    pub fn ocm2aflt2a(&self) -> Ocm2aflt2aR {
        Ocm2aflt2aR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc CCP2A"]
    #[inline(always)]
    pub fn ccp2a(&self) -> Ccp2aR {
        Ccp2aR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc OCM2BFLT2B"]
    #[inline(always)]
    pub fn ocm2bflt2b(&self) -> Ocm2bflt2bR {
        Ocm2bflt2bR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc CCP2B"]
    #[inline(always)]
    pub fn ccp2b(&self) -> Ccp2bR {
        Ccp2bR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - desc OCM3AFLT3A"]
    #[inline(always)]
    pub fn ocm3aflt3a(&self) -> Ocm3aflt3aR {
        Ocm3aflt3aR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - desc CCP3A"]
    #[inline(always)]
    pub fn ccp3a(&self) -> Ccp3aR {
        Ccp3aR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - desc OCM3BFLT3B"]
    #[inline(always)]
    pub fn ocm3bflt3b(&self) -> Ocm3bflt3bR {
        Ocm3bflt3bR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - desc CCP3B"]
    #[inline(always)]
    pub fn ccp3b(&self) -> Ccp3bR {
        Ccp3bR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - desc FLTBK"]
    #[inline(always)]
    pub fn fltbk(&self) -> FltbkR {
        FltbkR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - desc BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - desc FLTET"]
    #[inline(always)]
    pub fn fltet(&self) -> FltetR {
        FltetR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - desc ETP"]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc OCM1AFLT1A"]
    #[inline(always)]
    pub fn ocm1aflt1a(&mut self) -> Ocm1aflt1aW<'_, FltrSpec> {
        Ocm1aflt1aW::new(self, 0)
    }
    #[doc = "Bit 3 - desc CCP1A"]
    #[inline(always)]
    pub fn ccp1a(&mut self) -> Ccp1aW<'_, FltrSpec> {
        Ccp1aW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc OCM1BFLT1B"]
    #[inline(always)]
    pub fn ocm1bflt1b(&mut self) -> Ocm1bflt1bW<'_, FltrSpec> {
        Ocm1bflt1bW::new(self, 4)
    }
    #[doc = "Bit 7 - desc CCP1B"]
    #[inline(always)]
    pub fn ccp1b(&mut self) -> Ccp1bW<'_, FltrSpec> {
        Ccp1bW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc OCM2AFLT2A"]
    #[inline(always)]
    pub fn ocm2aflt2a(&mut self) -> Ocm2aflt2aW<'_, FltrSpec> {
        Ocm2aflt2aW::new(self, 8)
    }
    #[doc = "Bit 11 - desc CCP2A"]
    #[inline(always)]
    pub fn ccp2a(&mut self) -> Ccp2aW<'_, FltrSpec> {
        Ccp2aW::new(self, 11)
    }
    #[doc = "Bits 12:14 - desc OCM2BFLT2B"]
    #[inline(always)]
    pub fn ocm2bflt2b(&mut self) -> Ocm2bflt2bW<'_, FltrSpec> {
        Ocm2bflt2bW::new(self, 12)
    }
    #[doc = "Bit 15 - desc CCP2B"]
    #[inline(always)]
    pub fn ccp2b(&mut self) -> Ccp2bW<'_, FltrSpec> {
        Ccp2bW::new(self, 15)
    }
    #[doc = "Bits 16:18 - desc OCM3AFLT3A"]
    #[inline(always)]
    pub fn ocm3aflt3a(&mut self) -> Ocm3aflt3aW<'_, FltrSpec> {
        Ocm3aflt3aW::new(self, 16)
    }
    #[doc = "Bit 19 - desc CCP3A"]
    #[inline(always)]
    pub fn ccp3a(&mut self) -> Ccp3aW<'_, FltrSpec> {
        Ccp3aW::new(self, 19)
    }
    #[doc = "Bits 20:22 - desc OCM3BFLT3B"]
    #[inline(always)]
    pub fn ocm3bflt3b(&mut self) -> Ocm3bflt3bW<'_, FltrSpec> {
        Ocm3bflt3bW::new(self, 20)
    }
    #[doc = "Bit 23 - desc CCP3B"]
    #[inline(always)]
    pub fn ccp3b(&mut self) -> Ccp3bW<'_, FltrSpec> {
        Ccp3bW::new(self, 23)
    }
    #[doc = "Bits 24:26 - desc FLTBK"]
    #[inline(always)]
    pub fn fltbk(&mut self) -> FltbkW<'_, FltrSpec> {
        FltbkW::new(self, 24)
    }
    #[doc = "Bit 27 - desc BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, FltrSpec> {
        BkpW::new(self, 27)
    }
    #[doc = "Bits 28:30 - desc FLTET"]
    #[inline(always)]
    pub fn fltet(&mut self) -> FltetW<'_, FltrSpec> {
        FltetW::new(self, 28)
    }
    #[doc = "Bit 31 - desc ETP"]
    #[inline(always)]
    pub fn etp(&mut self) -> EtpW<'_, FltrSpec> {
        EtpW::new(self, 31)
    }
}
#[doc = "desc FLTR\n\nYou can [`read`](crate::Reg::read) this register and get [`fltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltrSpec;
impl crate::RegisterSpec for FltrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltr::R`](R) reader structure"]
impl crate::Readable for FltrSpec {}
#[doc = "`write(|w| ..)` method takes [`fltr::W`](W) writer structure"]
impl crate::Writable for FltrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTR to value 0"]
impl crate::Resettable for FltrSpec {}
