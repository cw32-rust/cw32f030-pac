#[doc = "Register `GTIMETR` reader"]
pub type R = crate::R<GtimetrSpec>;
#[doc = "Register `GTIMETR` writer"]
pub type W = crate::W<GtimetrSpec>;
#[doc = "Field `GTIM1ETR` reader - desc GTIM1ETR"]
pub type Gtim1etrR = crate::FieldReader;
#[doc = "Field `GTIM1ETR` writer - desc GTIM1ETR"]
pub type Gtim1etrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTIM2ETR` reader - desc GTIM2ETR"]
pub type Gtim2etrR = crate::FieldReader;
#[doc = "Field `GTIM2ETR` writer - desc GTIM2ETR"]
pub type Gtim2etrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTIM3ETR` reader - desc GTIM3ETR"]
pub type Gtim3etrR = crate::FieldReader;
#[doc = "Field `GTIM3ETR` writer - desc GTIM3ETR"]
pub type Gtim3etrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTIM4ETR` reader - desc GTIM4ETR"]
pub type Gtim4etrR = crate::FieldReader;
#[doc = "Field `GTIM4ETR` writer - desc GTIM4ETR"]
pub type Gtim4etrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc GTIM1ETR"]
    #[inline(always)]
    pub fn gtim1etr(&self) -> Gtim1etrR {
        Gtim1etrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc GTIM2ETR"]
    #[inline(always)]
    pub fn gtim2etr(&self) -> Gtim2etrR {
        Gtim2etrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc GTIM3ETR"]
    #[inline(always)]
    pub fn gtim3etr(&self) -> Gtim3etrR {
        Gtim3etrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc GTIM4ETR"]
    #[inline(always)]
    pub fn gtim4etr(&self) -> Gtim4etrR {
        Gtim4etrR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc GTIM1ETR"]
    #[inline(always)]
    pub fn gtim1etr(&mut self) -> Gtim1etrW<'_, GtimetrSpec> {
        Gtim1etrW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc GTIM2ETR"]
    #[inline(always)]
    pub fn gtim2etr(&mut self) -> Gtim2etrW<'_, GtimetrSpec> {
        Gtim2etrW::new(self, 4)
    }
    #[doc = "Bits 8:10 - desc GTIM3ETR"]
    #[inline(always)]
    pub fn gtim3etr(&mut self) -> Gtim3etrW<'_, GtimetrSpec> {
        Gtim3etrW::new(self, 8)
    }
    #[doc = "Bits 12:14 - desc GTIM4ETR"]
    #[inline(always)]
    pub fn gtim4etr(&mut self) -> Gtim4etrW<'_, GtimetrSpec> {
        Gtim4etrW::new(self, 12)
    }
}
#[doc = "GTIM1-4 ETR Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`gtimetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtimetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtimetrSpec;
impl crate::RegisterSpec for GtimetrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtimetr::R`](R) reader structure"]
impl crate::Readable for GtimetrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtimetr::W`](W) writer structure"]
impl crate::Writable for GtimetrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTIMETR to value 0"]
impl crate::Resettable for GtimetrSpec {}
