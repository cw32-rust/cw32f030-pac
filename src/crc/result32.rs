#[doc = "Register `RESULT32` reader"]
pub type R = crate::R<Result32Spec>;
#[doc = "Field `RESULT32` reader - desc RESULT32"]
pub type Result32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc RESULT32"]
    #[inline(always)]
    pub fn result32(&self) -> Result32R {
        Result32R::new(self.bits)
    }
}
#[doc = "Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result32::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Result32Spec;
impl crate::RegisterSpec for Result32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result32::R`](R) reader structure"]
impl crate::Readable for Result32Spec {}
#[doc = "`reset()` method sets RESULT32 to value 0"]
impl crate::Resettable for Result32Spec {}
