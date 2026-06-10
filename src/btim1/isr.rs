#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `OV` reader - desc OV"]
pub type OvR = crate::BitReader;
#[doc = "Field `TI` reader - desc TI"]
pub type TiR = crate::BitReader;
#[doc = "Field `TOP` reader - desc TOP"]
pub type TopR = crate::BitReader;
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
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
