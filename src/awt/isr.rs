#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `UD` reader - desc UD"]
pub type UdR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UdR {
        UdR::new(((self.bits >> 3) & 1) != 0)
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
