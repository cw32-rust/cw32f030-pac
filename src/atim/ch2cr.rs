#[doc = "Register `CH2CR` reader"]
pub type R = crate::R<Ch2crSpec>;
#[doc = "Register `CH2CR` writer"]
pub type W = crate::W<Ch2crSpec>;
#[doc = "Field `BKSA` reader - desc BKSA"]
pub type BksaR = crate::FieldReader;
#[doc = "Field `BKSA` writer - desc BKSA"]
pub type BksaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BKSB` reader - desc BKSB"]
pub type BksbR = crate::FieldReader;
#[doc = "Field `BKSB` writer - desc BKSB"]
pub type BksbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSA` reader - desc CSA"]
pub type CsaR = crate::BitReader;
#[doc = "Field `CSA` writer - desc CSA"]
pub type CsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSB` reader - desc CSB"]
pub type CsbR = crate::BitReader;
#[doc = "Field `CSB` writer - desc CSB"]
pub type CsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFEA` reader - desc BUFEA"]
pub type BufeaR = crate::BitReader;
#[doc = "Field `BUFEA` writer - desc BUFEA"]
pub type BufeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFEB` reader - desc BUFEB"]
pub type BufebR = crate::BitReader;
#[doc = "Field `BUFEB` writer - desc BUFEB"]
pub type BufebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIEA` reader - desc CIEA"]
pub type CieaR = crate::BitReader;
#[doc = "Field `CIEA` writer - desc CIEA"]
pub type CieaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIEB` reader - desc CIEB"]
pub type CiebR = crate::BitReader;
#[doc = "Field `CIEB` writer - desc CIEB"]
pub type CiebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEA` reader - desc CDEA"]
pub type CdeaR = crate::BitReader;
#[doc = "Field `CDEA` writer - desc CDEA"]
pub type CdeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEB` reader - desc CDEB"]
pub type CdebR = crate::BitReader;
#[doc = "Field `CDEB` writer - desc CDEB"]
pub type CdebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CISB` reader - desc CISB"]
pub type CisbR = crate::FieldReader;
#[doc = "Field `CISB` writer - desc CISB"]
pub type CisbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCGA` reader - desc CCGA"]
pub type CcgaR = crate::BitReader;
#[doc = "Field `CCGA` writer - desc CCGA"]
pub type CcgaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCGB` reader - desc CCGB"]
pub type CcgbR = crate::BitReader;
#[doc = "Field `CCGB` writer - desc CCGB"]
pub type CcgbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc BKSA"]
    #[inline(always)]
    pub fn bksa(&self) -> BksaR {
        BksaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc BKSB"]
    #[inline(always)]
    pub fn bksb(&self) -> BksbR {
        BksbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc CSA"]
    #[inline(always)]
    pub fn csa(&self) -> CsaR {
        CsaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CSB"]
    #[inline(always)]
    pub fn csb(&self) -> CsbR {
        CsbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BUFEA"]
    #[inline(always)]
    pub fn bufea(&self) -> BufeaR {
        BufeaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BUFEB"]
    #[inline(always)]
    pub fn bufeb(&self) -> BufebR {
        BufebR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CIEA"]
    #[inline(always)]
    pub fn ciea(&self) -> CieaR {
        CieaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CIEB"]
    #[inline(always)]
    pub fn cieb(&self) -> CiebR {
        CiebR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CDEA"]
    #[inline(always)]
    pub fn cdea(&self) -> CdeaR {
        CdeaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CDEB"]
    #[inline(always)]
    pub fn cdeb(&self) -> CdebR {
        CdebR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - desc CISB"]
    #[inline(always)]
    pub fn cisb(&self) -> CisbR {
        CisbR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc CCGA"]
    #[inline(always)]
    pub fn ccga(&self) -> CcgaR {
        CcgaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc CCGB"]
    #[inline(always)]
    pub fn ccgb(&self) -> CcgbR {
        CcgbR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc BKSA"]
    #[inline(always)]
    pub fn bksa(&mut self) -> BksaW<'_, Ch2crSpec> {
        BksaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc BKSB"]
    #[inline(always)]
    pub fn bksb(&mut self) -> BksbW<'_, Ch2crSpec> {
        BksbW::new(self, 2)
    }
    #[doc = "Bit 4 - desc CSA"]
    #[inline(always)]
    pub fn csa(&mut self) -> CsaW<'_, Ch2crSpec> {
        CsaW::new(self, 4)
    }
    #[doc = "Bit 5 - desc CSB"]
    #[inline(always)]
    pub fn csb(&mut self) -> CsbW<'_, Ch2crSpec> {
        CsbW::new(self, 5)
    }
    #[doc = "Bit 6 - desc BUFEA"]
    #[inline(always)]
    pub fn bufea(&mut self) -> BufeaW<'_, Ch2crSpec> {
        BufeaW::new(self, 6)
    }
    #[doc = "Bit 7 - desc BUFEB"]
    #[inline(always)]
    pub fn bufeb(&mut self) -> BufebW<'_, Ch2crSpec> {
        BufebW::new(self, 7)
    }
    #[doc = "Bit 8 - desc CIEA"]
    #[inline(always)]
    pub fn ciea(&mut self) -> CieaW<'_, Ch2crSpec> {
        CieaW::new(self, 8)
    }
    #[doc = "Bit 9 - desc CIEB"]
    #[inline(always)]
    pub fn cieb(&mut self) -> CiebW<'_, Ch2crSpec> {
        CiebW::new(self, 9)
    }
    #[doc = "Bit 10 - desc CDEA"]
    #[inline(always)]
    pub fn cdea(&mut self) -> CdeaW<'_, Ch2crSpec> {
        CdeaW::new(self, 10)
    }
    #[doc = "Bit 11 - desc CDEB"]
    #[inline(always)]
    pub fn cdeb(&mut self) -> CdebW<'_, Ch2crSpec> {
        CdebW::new(self, 11)
    }
    #[doc = "Bits 12:13 - desc CISB"]
    #[inline(always)]
    pub fn cisb(&mut self) -> CisbW<'_, Ch2crSpec> {
        CisbW::new(self, 12)
    }
    #[doc = "Bit 14 - desc CCGA"]
    #[inline(always)]
    pub fn ccga(&mut self) -> CcgaW<'_, Ch2crSpec> {
        CcgaW::new(self, 14)
    }
    #[doc = "Bit 15 - desc CCGB"]
    #[inline(always)]
    pub fn ccgb(&mut self) -> CcgbW<'_, Ch2crSpec> {
        CcgbW::new(self, 15)
    }
}
#[doc = "desc CH2CR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2crSpec;
impl crate::RegisterSpec for Ch2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2cr::R`](R) reader structure"]
impl crate::Readable for Ch2crSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2cr::W`](W) writer structure"]
impl crate::Writable for Ch2crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2CR to value 0"]
impl crate::Resettable for Ch2crSpec {}
