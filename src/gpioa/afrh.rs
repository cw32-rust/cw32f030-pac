#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Field `AFR8` reader - desc AFR8"]
pub type Afr8R = crate::FieldReader;
#[doc = "Field `AFR8` writer - desc AFR8"]
pub type Afr8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR9` reader - desc AFR9"]
pub type Afr9R = crate::FieldReader;
#[doc = "Field `AFR9` writer - desc AFR9"]
pub type Afr9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR10` reader - desc AFR10"]
pub type Afr10R = crate::FieldReader;
#[doc = "Field `AFR10` writer - desc AFR10"]
pub type Afr10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR11` reader - desc AFR11"]
pub type Afr11R = crate::FieldReader;
#[doc = "Field `AFR11` writer - desc AFR11"]
pub type Afr11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR12` reader - desc AFR12"]
pub type Afr12R = crate::FieldReader;
#[doc = "Field `AFR12` writer - desc AFR12"]
pub type Afr12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 0:3 - desc AFR8"]
    #[inline(always)]
    pub fn afr8(&self) -> Afr8R {
        Afr8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc AFR9"]
    #[inline(always)]
    pub fn afr9(&self) -> Afr9R {
        Afr9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc AFR10"]
    #[inline(always)]
    pub fn afr10(&self) -> Afr10R {
        Afr10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc AFR11"]
    #[inline(always)]
    pub fn afr11(&self) -> Afr11R {
        Afr11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc AFR12"]
    #[inline(always)]
    pub fn afr12(&self) -> Afr12R {
        Afr12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
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
    #[doc = "Bits 0:3 - desc AFR8"]
    #[inline(always)]
    pub fn afr8(&mut self) -> Afr8W<'_, AfrhSpec> {
        Afr8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc AFR9"]
    #[inline(always)]
    pub fn afr9(&mut self) -> Afr9W<'_, AfrhSpec> {
        Afr9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc AFR10"]
    #[inline(always)]
    pub fn afr10(&mut self) -> Afr10W<'_, AfrhSpec> {
        Afr10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc AFR11"]
    #[inline(always)]
    pub fn afr11(&mut self) -> Afr11W<'_, AfrhSpec> {
        Afr11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - desc AFR12"]
    #[inline(always)]
    pub fn afr12(&mut self) -> Afr12W<'_, AfrhSpec> {
        Afr12W::new(self, 16)
    }
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
