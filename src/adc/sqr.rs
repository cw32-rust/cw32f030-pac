#[doc = "Register `SQR` reader"]
pub type R = crate::R<SqrSpec>;
#[doc = "Register `SQR` writer"]
pub type W = crate::W<SqrSpec>;
#[doc = "Field `SQR0` reader - desc SQR0"]
pub type Sqr0R = crate::FieldReader;
#[doc = "Field `SQR0` writer - desc SQR0"]
pub type Sqr0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQR1` reader - desc SQR1"]
pub type Sqr1R = crate::FieldReader;
#[doc = "Field `SQR1` writer - desc SQR1"]
pub type Sqr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQR2` reader - desc SQR2"]
pub type Sqr2R = crate::FieldReader;
#[doc = "Field `SQR2` writer - desc SQR2"]
pub type Sqr2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQR3` reader - desc SQR3"]
pub type Sqr3R = crate::FieldReader;
#[doc = "Field `SQR3` writer - desc SQR3"]
pub type Sqr3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENS` reader - desc ENS"]
pub type EnsR = crate::FieldReader;
#[doc = "Field `ENS` writer - desc ENS"]
pub type EnsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - desc SQR0"]
    #[inline(always)]
    pub fn sqr0(&self) -> Sqr0R {
        Sqr0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc SQR1"]
    #[inline(always)]
    pub fn sqr1(&self) -> Sqr1R {
        Sqr1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc SQR2"]
    #[inline(always)]
    pub fn sqr2(&self) -> Sqr2R {
        Sqr2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc SQR3"]
    #[inline(always)]
    pub fn sqr3(&self) -> Sqr3R {
        Sqr3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - desc ENS"]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SQR0"]
    #[inline(always)]
    pub fn sqr0(&mut self) -> Sqr0W<'_, SqrSpec> {
        Sqr0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc SQR1"]
    #[inline(always)]
    pub fn sqr1(&mut self) -> Sqr1W<'_, SqrSpec> {
        Sqr1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc SQR2"]
    #[inline(always)]
    pub fn sqr2(&mut self) -> Sqr2W<'_, SqrSpec> {
        Sqr2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc SQR3"]
    #[inline(always)]
    pub fn sqr3(&mut self) -> Sqr3W<'_, SqrSpec> {
        Sqr3W::new(self, 12)
    }
    #[doc = "Bits 16:17 - desc ENS"]
    #[inline(always)]
    pub fn ens(&mut self) -> EnsW<'_, SqrSpec> {
        EnsW::new(self, 16)
    }
}
#[doc = "desc SQR\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SqrSpec;
impl crate::RegisterSpec for SqrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr::R`](R) reader structure"]
impl crate::Readable for SqrSpec {}
#[doc = "`write(|w| ..)` method takes [`sqr::W`](W) writer structure"]
impl crate::Writable for SqrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SQR to value 0"]
impl crate::Resettable for SqrSpec {}
