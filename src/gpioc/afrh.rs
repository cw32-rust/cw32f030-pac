#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Field `AFR13` reader - desc AFR13"]
pub type Afr13R = crate::FieldReader;
#[doc = "Field `AFR13` writer - desc AFR13"]
pub type Afr13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR14` reader - desc AFR14"]
pub type Afr14R = crate::FieldReader;
#[doc = "Field `AFR14` writer - desc AFR14"]
pub type Afr14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR15` reader - desc AFR15"]
pub type Afr15R = crate::FieldReader;
#[doc = "Field `AFR15` writer - desc AFR15"]
pub type Afr15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 20:23 - desc AFR13"]
    #[inline(always)]
    pub fn afr13(&self) -> Afr13R {
        Afr13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc AFR14"]
    #[inline(always)]
    pub fn afr14(&self) -> Afr14R {
        Afr14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc AFR15"]
    #[inline(always)]
    pub fn afr15(&self) -> Afr15R {
        Afr15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - desc AFR13"]
    #[inline(always)]
    pub fn afr13(&mut self) -> Afr13W<'_, AfrhSpec> {
        Afr13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - desc AFR14"]
    #[inline(always)]
    pub fn afr14(&mut self) -> Afr14W<'_, AfrhSpec> {
        Afr14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - desc AFR15"]
    #[inline(always)]
    pub fn afr15(&mut self) -> Afr15W<'_, AfrhSpec> {
        Afr15W::new(self, 28)
    }
}
#[doc = "desc AFRH\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrhSpec;
impl crate::RegisterSpec for AfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AfrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AfrhSpec {}
