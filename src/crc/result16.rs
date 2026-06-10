#[doc = "Register `RESULT16` reader"]
pub type R = crate::R<Result16Spec>;
#[doc = "Field `RESULT16` reader - desc RESULT16"]
pub type Result16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc RESULT16"]
    #[inline(always)]
    pub fn result16(&self) -> Result16R {
        Result16R::new(self.bits)
    }
}
#[doc = "Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`result16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Result16Spec;
impl crate::RegisterSpec for Result16Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`result16::R`](R) reader structure"]
impl crate::Readable for Result16Spec {}
#[doc = "`reset()` method sets RESULT16 to value 0"]
impl crate::Resettable for Result16Spec {}
