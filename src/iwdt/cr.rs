#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PrsR = crate::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PrsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACTION` reader - desc ACTION"]
pub type ActionR = crate::BitReader;
#[doc = "Field `ACTION` writer - desc ACTION"]
pub type ActionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSE` reader - desc PAUSE"]
pub type PauseR = crate::BitReader;
#[doc = "Field `PAUSE` writer - desc PAUSE"]
pub type PauseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc ACTION"]
    #[inline(always)]
    pub fn action(&self) -> ActionR {
        ActionR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PAUSE"]
    #[inline(always)]
    pub fn pause(&self) -> PauseR {
        PauseR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, CrSpec> {
        PrsW::new(self, 0)
    }
    #[doc = "Bit 3 - desc ACTION"]
    #[inline(always)]
    pub fn action(&mut self) -> ActionW<'_, CrSpec> {
        ActionW::new(self, 3)
    }
    #[doc = "Bit 4 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, CrSpec> {
        IeW::new(self, 4)
    }
    #[doc = "Bit 5 - desc PAUSE"]
    #[inline(always)]
    pub fn pause(&mut self) -> PauseW<'_, CrSpec> {
        PauseW::new(self, 5)
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
