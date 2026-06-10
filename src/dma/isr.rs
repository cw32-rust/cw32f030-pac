#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TC1` reader - desc TC1"]
pub type Tc1R = crate::BitReader;
#[doc = "Field `TE1` reader - desc TE1"]
pub type Te1R = crate::BitReader;
#[doc = "Field `TC2` reader - desc TC2"]
pub type Tc2R = crate::BitReader;
#[doc = "Field `TE2` reader - desc TE2"]
pub type Te2R = crate::BitReader;
#[doc = "Field `TC3` reader - desc TC3"]
pub type Tc3R = crate::BitReader;
#[doc = "Field `TE3` reader - desc TE3"]
pub type Te3R = crate::BitReader;
#[doc = "Field `TC4` reader - desc TC4"]
pub type Tc4R = crate::BitReader;
#[doc = "Field `TE4` reader - desc TE4"]
pub type Te4R = crate::BitReader;
#[doc = "Field `TC5` reader - desc TC5"]
pub type Tc5R = crate::BitReader;
#[doc = "Field `TE5` reader - desc TE5"]
pub type Te5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc TC1"]
    #[inline(always)]
    pub fn tc1(&self) -> Tc1R {
        Tc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TE1"]
    #[inline(always)]
    pub fn te1(&self) -> Te1R {
        Te1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TC2"]
    #[inline(always)]
    pub fn tc2(&self) -> Tc2R {
        Tc2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TE2"]
    #[inline(always)]
    pub fn te2(&self) -> Te2R {
        Te2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TC3"]
    #[inline(always)]
    pub fn tc3(&self) -> Tc3R {
        Tc3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TE3"]
    #[inline(always)]
    pub fn te3(&self) -> Te3R {
        Te3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TC4"]
    #[inline(always)]
    pub fn tc4(&self) -> Tc4R {
        Tc4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TE4"]
    #[inline(always)]
    pub fn te4(&self) -> Te4R {
        Te4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - desc TC5"]
    #[inline(always)]
    pub fn tc5(&self) -> Tc5R {
        Tc5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc TE5"]
    #[inline(always)]
    pub fn te5(&self) -> Te5R {
        Te5R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
