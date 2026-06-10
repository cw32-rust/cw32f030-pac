#[doc = "Register `DTR` reader"]
pub type R = crate::R<DtrSpec>;
#[doc = "Register `DTR` writer"]
pub type W = crate::W<DtrSpec>;
#[doc = "Field `DTR` reader - desc DTR"]
pub type DtrR = crate::FieldReader;
#[doc = "Field `DTR` writer - desc DTR"]
pub type DtrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTEN` reader - desc DTEN"]
pub type DtenR = crate::BitReader;
#[doc = "Field `DTEN` writer - desc DTEN"]
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKE` reader - desc BKE"]
pub type BkeR = crate::BitReader;
#[doc = "Field `BKE` writer - desc BKE"]
pub type BkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOE` reader - desc AOE"]
pub type AoeR = crate::BitReader;
#[doc = "Field `AOE` writer - desc AOE"]
pub type AoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOE` reader - desc MOE"]
pub type MoeR = crate::BitReader;
#[doc = "Field `MOE` writer - desc MOE"]
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAFEEN` reader - desc SAFEEN"]
pub type SafeenR = crate::BitReader;
#[doc = "Field `SAFEEN` writer - desc SAFEEN"]
pub type SafeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCE` reader - desc VCE"]
pub type VceR = crate::BitReader;
#[doc = "Field `VCE` writer - desc VCE"]
pub type VceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - desc DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - desc DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BKE"]
    #[inline(always)]
    pub fn bke(&self) -> BkeR {
        BkeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&self) -> AoeR {
        AoeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SAFEEN"]
    #[inline(always)]
    pub fn safeen(&self) -> SafeenR {
        SafeenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc VCE"]
    #[inline(always)]
    pub fn vce(&self) -> VceR {
        VceR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DTR"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<'_, DtrSpec> {
        DtrW::new(self, 0)
    }
    #[doc = "Bit 9 - desc DTEN"]
    #[inline(always)]
    pub fn dten(&mut self) -> DtenW<'_, DtrSpec> {
        DtenW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BKE"]
    #[inline(always)]
    pub fn bke(&mut self) -> BkeW<'_, DtrSpec> {
        BkeW::new(self, 10)
    }
    #[doc = "Bit 11 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AoeW<'_, DtrSpec> {
        AoeW::new(self, 11)
    }
    #[doc = "Bit 12 - desc MOE"]
    #[inline(always)]
    pub fn moe(&mut self) -> MoeW<'_, DtrSpec> {
        MoeW::new(self, 12)
    }
    #[doc = "Bit 13 - desc SAFEEN"]
    #[inline(always)]
    pub fn safeen(&mut self) -> SafeenW<'_, DtrSpec> {
        SafeenW::new(self, 13)
    }
    #[doc = "Bit 14 - desc VCE"]
    #[inline(always)]
    pub fn vce(&mut self) -> VceW<'_, DtrSpec> {
        VceW::new(self, 14)
    }
}
#[doc = "desc DTR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtrSpec;
impl crate::RegisterSpec for DtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtr::R`](R) reader structure"]
impl crate::Readable for DtrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtr::W`](W) writer structure"]
impl crate::Writable for DtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTR to value 0"]
impl crate::Resettable for DtrSpec {}
