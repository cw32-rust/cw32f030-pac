#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `OV` reader - desc OV"]
pub type OvR = crate::BitReader;
#[doc = "Field `TI` reader - desc TI"]
pub type TiR = crate::BitReader;
#[doc = "Field `UD` reader - desc UD"]
pub type UdR = crate::BitReader;
#[doc = "Field `CC1` reader - desc CC1"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC2` reader - desc CC2"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC3` reader - desc CC3"]
pub type Cc3R = crate::BitReader;
#[doc = "Field `CC4` reader - desc CC4"]
pub type Cc4R = crate::BitReader;
#[doc = "Field `DIRCHANGE` reader - desc DIRCHANGE"]
pub type DirchangeR = crate::BitReader;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DirR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OvR {
        OvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TI"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UdR {
        UdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC1"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC2"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CC3"]
    #[inline(always)]
    pub fn cc3(&self) -> Cc3R {
        Cc3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CC4"]
    #[inline(always)]
    pub fn cc4(&self) -> Cc4R {
        Cc4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DIRCHANGE"]
    #[inline(always)]
    pub fn dirchange(&self) -> DirchangeR {
        DirchangeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
