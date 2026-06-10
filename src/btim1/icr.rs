#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `OV` reader - desc OV"]
pub type OvR = crate::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI` reader - desc TI"]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - desc TI"]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOP` reader - desc TOP"]
pub type TopR = crate::BitReader;
#[doc = "Field `TOP` writer - desc TOP"]
pub type TopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OvR {
        OvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TI"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TOP"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    pub fn ov(&mut self) -> OvW<'_, IcrSpec> {
        OvW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TI"]
    #[inline(always)]
    pub fn ti(&mut self) -> TiW<'_, IcrSpec> {
        TiW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TOP"]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<'_, IcrSpec> {
        TopW::new(self, 2)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
