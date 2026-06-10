#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
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
    pub fn ov(&mut self) -> OvW<'_, IerSpec> {
        OvW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TI"]
    #[inline(always)]
    pub fn ti(&mut self) -> TiW<'_, IerSpec> {
        TiW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TOP"]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<'_, IerSpec> {
        TopW::new(self, 2)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
