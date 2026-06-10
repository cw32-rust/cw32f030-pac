#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD` reader - desc MD"]
pub type MdR = crate::FieldReader;
#[doc = "Field `MD` writer - desc MD"]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PrsR = crate::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PrsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRC` reader - desc SRC"]
pub type SrcR = crate::FieldReader;
#[doc = "Field `SRC` writer - desc SRC"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc MD"]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - desc SRC"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc MD"]
    #[inline(always)]
    pub fn md(&mut self) -> MdW<'_, CrSpec> {
        MdW::new(self, 1)
    }
    #[doc = "Bits 4:7 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, CrSpec> {
        PrsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - desc SRC"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, CrSpec> {
        SrcW::new(self, 8)
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
