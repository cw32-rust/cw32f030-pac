#[doc = "Register `TIMITR` reader"]
pub type R = crate::R<TimitrSpec>;
#[doc = "Register `TIMITR` writer"]
pub type W = crate::W<TimitrSpec>;
#[doc = "Field `ATIMITR` reader - desc ATIMITR"]
pub type AtimitrR = crate::FieldReader;
#[doc = "Field `ATIMITR` writer - desc ATIMITR"]
pub type AtimitrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTIM1ITR` reader - desc GTIM1ITR"]
pub type Gtim1itrR = crate::FieldReader;
#[doc = "Field `GTIM1ITR` writer - desc GTIM1ITR"]
pub type Gtim1itrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTIM2ITR` reader - desc GTIM2ITR"]
pub type Gtim2itrR = crate::FieldReader;
#[doc = "Field `GTIM2ITR` writer - desc GTIM2ITR"]
pub type Gtim2itrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTIM3ITR` reader - desc GTIM3ITR"]
pub type Gtim3itrR = crate::FieldReader;
#[doc = "Field `GTIM3ITR` writer - desc GTIM3ITR"]
pub type Gtim3itrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTIM4ITR` reader - desc GTIM4ITR"]
pub type Gtim4itrR = crate::FieldReader;
#[doc = "Field `GTIM4ITR` writer - desc GTIM4ITR"]
pub type Gtim4itrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BTIM1ITR` reader - desc BTIM1ITR"]
pub type Btim1itrR = crate::FieldReader;
#[doc = "Field `BTIM1ITR` writer - desc BTIM1ITR"]
pub type Btim1itrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BTIM2ITR` reader - desc BTIM2ITR"]
pub type Btim2itrR = crate::FieldReader;
#[doc = "Field `BTIM2ITR` writer - desc BTIM2ITR"]
pub type Btim2itrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BTIM3ITR` reader - desc BTIM3ITR"]
pub type Btim3itrR = crate::FieldReader;
#[doc = "Field `BTIM3ITR` writer - desc BTIM3ITR"]
pub type Btim3itrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc ATIMITR"]
    #[inline(always)]
    pub fn atimitr(&self) -> AtimitrR {
        AtimitrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc GTIM1ITR"]
    #[inline(always)]
    pub fn gtim1itr(&self) -> Gtim1itrR {
        Gtim1itrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - desc GTIM2ITR"]
    #[inline(always)]
    pub fn gtim2itr(&self) -> Gtim2itrR {
        Gtim2itrR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - desc GTIM3ITR"]
    #[inline(always)]
    pub fn gtim3itr(&self) -> Gtim3itrR {
        Gtim3itrR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc GTIM4ITR"]
    #[inline(always)]
    pub fn gtim4itr(&self) -> Gtim4itrR {
        Gtim4itrR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - desc BTIM1ITR"]
    #[inline(always)]
    pub fn btim1itr(&self) -> Btim1itrR {
        Btim1itrR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - desc BTIM2ITR"]
    #[inline(always)]
    pub fn btim2itr(&self) -> Btim2itrR {
        Btim2itrR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - desc BTIM3ITR"]
    #[inline(always)]
    pub fn btim3itr(&self) -> Btim3itrR {
        Btim3itrR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc ATIMITR"]
    #[inline(always)]
    pub fn atimitr(&mut self) -> AtimitrW<'_, TimitrSpec> {
        AtimitrW::new(self, 0)
    }
    #[doc = "Bits 3:5 - desc GTIM1ITR"]
    #[inline(always)]
    pub fn gtim1itr(&mut self) -> Gtim1itrW<'_, TimitrSpec> {
        Gtim1itrW::new(self, 3)
    }
    #[doc = "Bits 6:8 - desc GTIM2ITR"]
    #[inline(always)]
    pub fn gtim2itr(&mut self) -> Gtim2itrW<'_, TimitrSpec> {
        Gtim2itrW::new(self, 6)
    }
    #[doc = "Bits 9:11 - desc GTIM3ITR"]
    #[inline(always)]
    pub fn gtim3itr(&mut self) -> Gtim3itrW<'_, TimitrSpec> {
        Gtim3itrW::new(self, 9)
    }
    #[doc = "Bits 12:14 - desc GTIM4ITR"]
    #[inline(always)]
    pub fn gtim4itr(&mut self) -> Gtim4itrW<'_, TimitrSpec> {
        Gtim4itrW::new(self, 12)
    }
    #[doc = "Bits 15:17 - desc BTIM1ITR"]
    #[inline(always)]
    pub fn btim1itr(&mut self) -> Btim1itrW<'_, TimitrSpec> {
        Btim1itrW::new(self, 15)
    }
    #[doc = "Bits 18:20 - desc BTIM2ITR"]
    #[inline(always)]
    pub fn btim2itr(&mut self) -> Btim2itrW<'_, TimitrSpec> {
        Btim2itrW::new(self, 18)
    }
    #[doc = "Bits 21:23 - desc BTIM3ITR"]
    #[inline(always)]
    pub fn btim3itr(&mut self) -> Btim3itrW<'_, TimitrSpec> {
        Btim3itrW::new(self, 21)
    }
}
#[doc = "BTIMx GTIMx ATIM ITR Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`timitr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timitr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimitrSpec;
impl crate::RegisterSpec for TimitrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timitr::R`](R) reader structure"]
impl crate::Readable for TimitrSpec {}
#[doc = "`write(|w| ..)` method takes [`timitr::W`](W) writer structure"]
impl crate::Writable for TimitrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMITR to value 0"]
impl crate::Resettable for TimitrSpec {}
