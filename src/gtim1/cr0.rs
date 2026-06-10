#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
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
#[doc = "Field `ENCMODE` reader - desc ENCMODE"]
pub type EncmodeR = crate::FieldReader;
#[doc = "Field `ENCMODE` writer - desc ENCMODE"]
pub type EncmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENCRESET` reader - desc ENCRESET"]
pub type EncresetR = crate::FieldReader;
#[doc = "Field `ENCRESET` writer - desc ENCRESET"]
pub type EncresetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENCRELOAD` reader - desc ENCRELOAD"]
pub type EncreloadR = crate::FieldReader;
#[doc = "Field `ENCRELOAD` writer - desc ENCRELOAD"]
pub type EncreloadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 15:16 - desc ENCMODE"]
    #[inline(always)]
    pub fn encmode(&self) -> EncmodeR {
        EncmodeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - desc ENCRESET"]
    #[inline(always)]
    pub fn encreset(&self) -> EncresetR {
        EncresetR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - desc ENCRELOAD"]
    #[inline(always)]
    pub fn encreload(&self) -> EncreloadR {
        EncreloadR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cr0Spec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 3 - desc TRS"]
    #[inline(always)]
    pub fn trs(&mut self) -> TrsW<'_, Cr0Spec> {
        TrsW::new(self, 3)
    }
    #[doc = "Bit 4 - desc POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, Cr0Spec> {
        PolW::new(self, 4)
    }
    #[doc = "Bit 5 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<'_, Cr0Spec> {
        OneshotW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TOGEN"]
    #[inline(always)]
    pub fn togen(&mut self) -> TogenW<'_, Cr0Spec> {
        TogenW::new(self, 6)
    }
    #[doc = "Bits 7:10 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, Cr0Spec> {
        PrsW::new(self, 7)
    }
    #[doc = "Bits 15:16 - desc ENCMODE"]
    #[inline(always)]
    pub fn encmode(&mut self) -> EncmodeW<'_, Cr0Spec> {
        EncmodeW::new(self, 15)
    }
    #[doc = "Bits 17:18 - desc ENCRESET"]
    #[inline(always)]
    pub fn encreset(&mut self) -> EncresetW<'_, Cr0Spec> {
        EncresetW::new(self, 17)
    }
    #[doc = "Bits 19:20 - desc ENCRELOAD"]
    #[inline(always)]
    pub fn encreload(&mut self) -> EncreloadW<'_, Cr0Spec> {
        EncreloadW::new(self, 19)
    }
}
#[doc = "Control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
