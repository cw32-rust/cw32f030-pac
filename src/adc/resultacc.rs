#[doc = "Register `RESULTACC` reader"]
pub type R = crate::R<ResultaccSpec>;
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type ResultR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "desc RESULTACC\n\nYou can [`read`](crate::Reg::read) this register and get [`resultacc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultaccSpec;
impl crate::RegisterSpec for ResultaccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resultacc::R`](R) reader structure"]
impl crate::Readable for ResultaccSpec {}
#[doc = "`reset()` method sets RESULTACC to value 0"]
impl crate::Resettable for ResultaccSpec {}
