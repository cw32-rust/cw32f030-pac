#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `PARITY` reader - desc PARITY"]
pub type ParityR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc PARITY"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
