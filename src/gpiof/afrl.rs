#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Field `AFR0` reader - desc AFR0"]
pub type Afr0R = crate::FieldReader;
#[doc = "Field `AFR0` writer - desc AFR0"]
pub type Afr0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR1` reader - desc AFR1"]
pub type Afr1R = crate::FieldReader;
#[doc = "Field `AFR1` writer - desc AFR1"]
pub type Afr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR3` reader - desc AFR3"]
pub type Afr3R = crate::FieldReader;
#[doc = "Field `AFR3` writer - desc AFR3"]
pub type Afr3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR6` reader - desc AFR6"]
pub type Afr6R = crate::FieldReader;
#[doc = "Field `AFR6` writer - desc AFR6"]
pub type Afr6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR7` reader - desc AFR7"]
pub type Afr7R = crate::FieldReader;
#[doc = "Field `AFR7` writer - desc AFR7"]
pub type Afr7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc AFR0"]
    #[inline(always)]
    pub fn afr0(&self) -> Afr0R {
        Afr0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc AFR1"]
    #[inline(always)]
    pub fn afr1(&self) -> Afr1R {
        Afr1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc AFR3"]
    #[inline(always)]
    pub fn afr3(&self) -> Afr3R {
        Afr3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc AFR6"]
    #[inline(always)]
    pub fn afr6(&self) -> Afr6R {
        Afr6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc AFR7"]
    #[inline(always)]
    pub fn afr7(&self) -> Afr7R {
        Afr7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc AFR0"]
    #[inline(always)]
    pub fn afr0(&mut self) -> Afr0W<'_, AfrlSpec> {
        Afr0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc AFR1"]
    #[inline(always)]
    pub fn afr1(&mut self) -> Afr1W<'_, AfrlSpec> {
        Afr1W::new(self, 4)
    }
    #[doc = "Bits 12:15 - desc AFR3"]
    #[inline(always)]
    pub fn afr3(&mut self) -> Afr3W<'_, AfrlSpec> {
        Afr3W::new(self, 12)
    }
    #[doc = "Bits 24:27 - desc AFR6"]
    #[inline(always)]
    pub fn afr6(&mut self) -> Afr6W<'_, AfrlSpec> {
        Afr6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - desc AFR7"]
    #[inline(always)]
    pub fn afr7(&mut self) -> Afr7W<'_, AfrlSpec> {
        Afr7W::new(self, 28)
    }
}
#[doc = "desc AFRL\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AfrlSpec {}
