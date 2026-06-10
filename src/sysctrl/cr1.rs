#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `HSIEN` reader - desc HSIEN"]
pub type HsienR = crate::BitReader;
#[doc = "Field `HSIEN` writer - desc HSIEN"]
pub type HsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEEN` reader - desc HSEEN"]
pub type HseenR = crate::BitReader;
#[doc = "Field `HSEEN` writer - desc HSEEN"]
pub type HseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLEN` reader - desc PLLEN"]
pub type PllenR = crate::BitReader;
#[doc = "Field `PLLEN` writer - desc PLLEN"]
pub type PllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIEN` reader - desc LSIEN"]
pub type LsienR = crate::BitReader;
#[doc = "Field `LSIEN` writer - desc LSIEN"]
pub type LsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEEN` reader - desc LSEEN"]
pub type LseenR = crate::BitReader;
#[doc = "Field `LSEEN` writer - desc LSEEN"]
pub type LseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSELOCK` reader - desc LSELOCK"]
pub type LselockR = crate::BitReader;
#[doc = "Field `LSELOCK` writer - desc LSELOCK"]
pub type LselockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECCS` reader - desc LSECCS"]
pub type LseccsR = crate::BitReader;
#[doc = "Field `LSECCS` writer - desc LSECCS"]
pub type LseccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSECCS` reader - desc HSECCS"]
pub type HseccsR = crate::BitReader;
#[doc = "Field `HSECCS` writer - desc HSECCS"]
pub type HseccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKCCS` reader - desc CLKCCS"]
pub type ClkccsR = crate::BitReader;
#[doc = "Field `CLKCCS` writer - desc CLKCCS"]
pub type ClkccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - desc HSIEN"]
    #[inline(always)]
    pub fn hsien(&self) -> HsienR {
        HsienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSEEN"]
    #[inline(always)]
    pub fn hseen(&self) -> HseenR {
        HseenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PllenR {
        PllenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc LSIEN"]
    #[inline(always)]
    pub fn lsien(&self) -> LsienR {
        LsienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LSEEN"]
    #[inline(always)]
    pub fn lseen(&self) -> LseenR {
        LseenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LSELOCK"]
    #[inline(always)]
    pub fn lselock(&self) -> LselockR {
        LselockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LSECCS"]
    #[inline(always)]
    pub fn lseccs(&self) -> LseccsR {
        LseccsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HSECCS"]
    #[inline(always)]
    pub fn hseccs(&self) -> HseccsR {
        HseccsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CLKCCS"]
    #[inline(always)]
    pub fn clkccs(&self) -> ClkccsR {
        ClkccsR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSIEN"]
    #[inline(always)]
    pub fn hsien(&mut self) -> HsienW<'_, Cr1Spec> {
        HsienW::new(self, 0)
    }
    #[doc = "Bit 1 - desc HSEEN"]
    #[inline(always)]
    pub fn hseen(&mut self) -> HseenW<'_, Cr1Spec> {
        HseenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PLLEN"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PllenW<'_, Cr1Spec> {
        PllenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc LSIEN"]
    #[inline(always)]
    pub fn lsien(&mut self) -> LsienW<'_, Cr1Spec> {
        LsienW::new(self, 3)
    }
    #[doc = "Bit 4 - desc LSEEN"]
    #[inline(always)]
    pub fn lseen(&mut self) -> LseenW<'_, Cr1Spec> {
        LseenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc LSELOCK"]
    #[inline(always)]
    pub fn lselock(&mut self) -> LselockW<'_, Cr1Spec> {
        LselockW::new(self, 5)
    }
    #[doc = "Bit 6 - desc LSECCS"]
    #[inline(always)]
    pub fn lseccs(&mut self) -> LseccsW<'_, Cr1Spec> {
        LseccsW::new(self, 6)
    }
    #[doc = "Bit 7 - desc HSECCS"]
    #[inline(always)]
    pub fn hseccs(&mut self) -> HseccsW<'_, Cr1Spec> {
        HseccsW::new(self, 7)
    }
    #[doc = "Bit 8 - desc CLKCCS"]
    #[inline(always)]
    pub fn clkccs(&mut self) -> ClkccsW<'_, Cr1Spec> {
        ClkccsW::new(self, 8)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Cr1Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
