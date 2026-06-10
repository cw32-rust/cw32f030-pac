#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ACCEN` reader - desc ACCEN"]
pub type AccenR = crate::BitReader;
#[doc = "Field `ACCEN` writer - desc ACCEN"]
pub type AccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCRST` reader - desc ACCRST"]
pub type AccrstR = crate::BitReader;
#[doc = "Field `ACCRST` writer - desc ACCRST"]
pub type AccrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - desc ACCEN"]
    #[inline(always)]
    pub fn accen(&self) -> AccenR {
        AccenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ACCRST"]
    #[inline(always)]
    pub fn accrst(&self) -> AccrstR {
        AccrstR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, Cr2Spec> {
        CntW::new(self, 0)
    }
    #[doc = "Bit 8 - desc ACCEN"]
    #[inline(always)]
    pub fn accen(&mut self) -> AccenW<'_, Cr2Spec> {
        AccenW::new(self, 8)
    }
    #[doc = "Bit 9 - desc ACCRST"]
    #[inline(always)]
    pub fn accrst(&mut self) -> AccrstW<'_, Cr2Spec> {
        AccrstW::new(self, 9)
    }
}
#[doc = "Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
