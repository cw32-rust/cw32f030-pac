#[doc = "Register `RESULT2` reader"]
pub type R = crate::R<Result2Spec>;
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type ResultR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc RESULT2\n\nYou can [`read`](crate::Reg::read) this register and get [`result2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Result2Spec;
impl crate::RegisterSpec for Result2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result2::R`](R) reader structure"]
impl crate::Readable for Result2Spec {}
#[doc = "`reset()` method sets RESULT2 to value 0"]
impl crate::Resettable for Result2Spec {}
