#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - desc COMP"]
pub type CompR = crate::BitReader;
#[doc = "Field `COMP` writer - desc COMP"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CT` reader - desc CT"]
pub type CtR = crate::BitReader;
#[doc = "Field `CT` writer - desc CT"]
pub type CtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2S` reader - desc PWM2S"]
pub type Pwm2sR = crate::BitReader;
#[doc = "Field `PWM2S` writer - desc PWM2S"]
pub type Pwm2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PrsR = crate::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PrsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUFPEN` reader - desc BUFPEN"]
pub type BufpenR = crate::BitReader;
#[doc = "Field `BUFPEN` writer - desc BUFPEN"]
pub type BufpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIE` reader - desc UIE"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Please keep 10/11"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Please keep 10/11"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ONESHOT` reader - desc ONESHOT"]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - desc ONESHOT"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCCS` reader - desc OCCS"]
pub type OccsR = crate::BitReader;
#[doc = "Field `OCCS` writer - desc OCCS"]
pub type OccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - desc URS"]
pub type UrsR = crate::BitReader;
#[doc = "Field `URS` writer - desc URS"]
pub type UrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - desc TIE"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - desc TIE"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - desc BIE"]
pub type BieR = crate::BitReader;
#[doc = "Field `BIE` writer - desc BIE"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CISA` reader - desc CISA"]
pub type CisaR = crate::FieldReader;
#[doc = "Field `CISA` writer - desc CISA"]
pub type CisaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OCCE` reader - desc OCCE"]
pub type OcceR = crate::BitReader;
#[doc = "Field `OCCE` writer - desc OCCE"]
pub type OcceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` reader - desc TG"]
pub type TgR = crate::BitReader;
#[doc = "Field `TG` writer - desc TG"]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UG` reader - desc UG"]
pub type UgR = crate::BitReader;
#[doc = "Field `UG` writer - desc UG"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` reader - desc BG"]
pub type BgR = crate::BitReader;
#[doc = "Field `BG` writer - desc BG"]
pub type BgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVE` reader - desc OVE"]
pub type OveR = crate::BitReader;
#[doc = "Field `OVE` writer - desc OVE"]
pub type OveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDE` reader - desc UNDE"]
pub type UndeR = crate::BitReader;
#[doc = "Field `UNDE` writer - desc UNDE"]
pub type UndeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc COMP"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&self) -> CtR {
        CtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PWM2S"]
    #[inline(always)]
    pub fn pwm2s(&self) -> Pwm2sR {
        Pwm2sR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc BUFPEN"]
    #[inline(always)]
    pub fn bufpen(&self) -> BufpenR {
        BufpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Please keep 10/11"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OCCS"]
    #[inline(always)]
    pub fn occs(&self) -> OccsR {
        OccsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc URS"]
    #[inline(always)]
    pub fn urs(&self) -> UrsR {
        UrsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - desc TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - desc CISA"]
    #[inline(always)]
    pub fn cisa(&self) -> CisaR {
        CisaR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - desc OCCE"]
    #[inline(always)]
    pub fn occe(&self) -> OcceR {
        OcceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TG"]
    #[inline(always)]
    pub fn tg(&self) -> TgR {
        TgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc UG"]
    #[inline(always)]
    pub fn ug(&self) -> UgR {
        UgR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc BG"]
    #[inline(always)]
    pub fn bg(&self) -> BgR {
        BgR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc OVE"]
    #[inline(always)]
    pub fn ove(&self) -> OveR {
        OveR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc UNDE"]
    #[inline(always)]
    pub fn unde(&self) -> UndeR {
        UndeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc COMP"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, CrSpec> {
        CompW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&mut self) -> CtW<'_, CrSpec> {
        CtW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PWM2S"]
    #[inline(always)]
    pub fn pwm2s(&mut self) -> Pwm2sW<'_, CrSpec> {
        Pwm2sW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, CrSpec> {
        PrsW::new(self, 4)
    }
    #[doc = "Bit 7 - desc BUFPEN"]
    #[inline(always)]
    pub fn bufpen(&mut self) -> BufpenW<'_, CrSpec> {
        BufpenW::new(self, 7)
    }
    #[doc = "Bit 10 - desc UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, CrSpec> {
        UieW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Please keep 10/11"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CrSpec> {
        ModeW::new(self, 12)
    }
    #[doc = "Bit 14 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<'_, CrSpec> {
        OneshotW::new(self, 14)
    }
    #[doc = "Bit 16 - desc OCCS"]
    #[inline(always)]
    pub fn occs(&mut self) -> OccsW<'_, CrSpec> {
        OccsW::new(self, 16)
    }
    #[doc = "Bit 17 - desc URS"]
    #[inline(always)]
    pub fn urs(&mut self) -> UrsW<'_, CrSpec> {
        UrsW::new(self, 17)
    }
    #[doc = "Bit 19 - desc TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, CrSpec> {
        TieW::new(self, 19)
    }
    #[doc = "Bit 20 - desc BIE"]
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<'_, CrSpec> {
        BieW::new(self, 20)
    }
    #[doc = "Bits 21:22 - desc CISA"]
    #[inline(always)]
    pub fn cisa(&mut self) -> CisaW<'_, CrSpec> {
        CisaW::new(self, 21)
    }
    #[doc = "Bit 23 - desc OCCE"]
    #[inline(always)]
    pub fn occe(&mut self) -> OcceW<'_, CrSpec> {
        OcceW::new(self, 23)
    }
    #[doc = "Bit 24 - desc TG"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, CrSpec> {
        TgW::new(self, 24)
    }
    #[doc = "Bit 25 - desc UG"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<'_, CrSpec> {
        UgW::new(self, 25)
    }
    #[doc = "Bit 26 - desc BG"]
    #[inline(always)]
    pub fn bg(&mut self) -> BgW<'_, CrSpec> {
        BgW::new(self, 26)
    }
    #[doc = "Bit 27 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, CrSpec> {
        DirW::new(self, 27)
    }
    #[doc = "Bit 28 - desc OVE"]
    #[inline(always)]
    pub fn ove(&mut self) -> OveW<'_, CrSpec> {
        OveW::new(self, 28)
    }
    #[doc = "Bit 29 - desc UNDE"]
    #[inline(always)]
    pub fn unde(&mut self) -> UndeW<'_, CrSpec> {
        UndeW::new(self, 29)
    }
}
#[doc = "desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
