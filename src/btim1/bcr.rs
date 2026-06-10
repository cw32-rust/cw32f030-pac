#[doc = "Register `BCR` reader"]
pub type R = crate::R<BcrSpec>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BcrSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRS` reader - desc TRS"]
pub type TrsR = crate::BitReader;
#[doc = "Field `TRS` writer - desc TRS"]
pub type TrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - desc POL"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - desc POL"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONESHOT` reader - desc ONESHOT"]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - desc ONESHOT"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOGEN` reader - desc TOGEN"]
pub type TogenR = crate::BitReader;
#[doc = "Field `TOGEN` writer - desc TOGEN"]
pub type TogenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PrsR = crate::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PrsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRSSTATUS` reader - desc PRSSTATUS"]
pub type PrsstatusR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - desc TRS"]
    #[inline(always)]
    pub fn trs(&self) -> TrsR {
        TrsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POL"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TOGEN"]
    #[inline(always)]
    pub fn togen(&self) -> TogenR {
        TogenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - desc PRSSTATUS"]
    #[inline(always)]
    pub fn prsstatus(&self) -> PrsstatusR {
        PrsstatusR::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, BcrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, BcrSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 3 - desc TRS"]
    #[inline(always)]
    pub fn trs(&mut self) -> TrsW<'_, BcrSpec> {
        TrsW::new(self, 3)
    }
    #[doc = "Bit 4 - desc POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, BcrSpec> {
        PolW::new(self, 4)
    }
    #[doc = "Bit 5 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<'_, BcrSpec> {
        OneshotW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TOGEN"]
    #[inline(always)]
    pub fn togen(&mut self) -> TogenW<'_, BcrSpec> {
        TogenW::new(self, 6)
    }
    #[doc = "Bits 7:10 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, BcrSpec> {
        PrsW::new(self, 7)
    }
}
#[doc = "Base Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcrSpec;
impl crate::RegisterSpec for BcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BcrSpec {}
