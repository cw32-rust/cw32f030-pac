#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Field `ADDR` reader - desc ADDR"]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "Parity check error addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {}
