#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `TC1` reader - desc TC1"]
pub type Tc1R = crate::BitReader;
#[doc = "Field `TC1` writer - desc TC1"]
pub type Tc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1` reader - desc TE1"]
pub type Te1R = crate::BitReader;
#[doc = "Field `TE1` writer - desc TE1"]
pub type Te1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2` reader - desc TC2"]
pub type Tc2R = crate::BitReader;
#[doc = "Field `TC2` writer - desc TC2"]
pub type Tc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE2` reader - desc TE2"]
pub type Te2R = crate::BitReader;
#[doc = "Field `TE2` writer - desc TE2"]
pub type Te2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC3` reader - desc TC3"]
pub type Tc3R = crate::BitReader;
#[doc = "Field `TC3` writer - desc TC3"]
pub type Tc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE3` reader - desc TE3"]
pub type Te3R = crate::BitReader;
#[doc = "Field `TE3` writer - desc TE3"]
pub type Te3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC4` reader - desc TC4"]
pub type Tc4R = crate::BitReader;
#[doc = "Field `TC4` writer - desc TC4"]
pub type Tc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE4` reader - desc TE4"]
pub type Te4R = crate::BitReader;
#[doc = "Field `TE4` writer - desc TE4"]
pub type Te4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC5` reader - desc TC5"]
pub type Tc5R = crate::BitReader;
#[doc = "Field `TC5` writer - desc TC5"]
pub type Tc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE5` reader - desc TE5"]
pub type Te5R = crate::BitReader;
#[doc = "Field `TE5` writer - desc TE5"]
pub type Te5W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - desc TC1"]
    #[inline(always)]
    pub fn tc1(&mut self) -> Tc1W<'_, IcrSpec> {
        Tc1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc TE1"]
    #[inline(always)]
    pub fn te1(&mut self) -> Te1W<'_, IcrSpec> {
        Te1W::new(self, 1)
    }
    #[doc = "Bit 4 - desc TC2"]
    #[inline(always)]
    pub fn tc2(&mut self) -> Tc2W<'_, IcrSpec> {
        Tc2W::new(self, 4)
    }
    #[doc = "Bit 5 - desc TE2"]
    #[inline(always)]
    pub fn te2(&mut self) -> Te2W<'_, IcrSpec> {
        Te2W::new(self, 5)
    }
    #[doc = "Bit 8 - desc TC3"]
    #[inline(always)]
    pub fn tc3(&mut self) -> Tc3W<'_, IcrSpec> {
        Tc3W::new(self, 8)
    }
    #[doc = "Bit 9 - desc TE3"]
    #[inline(always)]
    pub fn te3(&mut self) -> Te3W<'_, IcrSpec> {
        Te3W::new(self, 9)
    }
    #[doc = "Bit 12 - desc TC4"]
    #[inline(always)]
    pub fn tc4(&mut self) -> Tc4W<'_, IcrSpec> {
        Tc4W::new(self, 12)
    }
    #[doc = "Bit 13 - desc TE4"]
    #[inline(always)]
    pub fn te4(&mut self) -> Te4W<'_, IcrSpec> {
        Te4W::new(self, 13)
    }
    #[doc = "Bit 16 - desc TC5"]
    #[inline(always)]
    pub fn tc5(&mut self) -> Tc5W<'_, IcrSpec> {
        Tc5W::new(self, 16)
    }
    #[doc = "Bit 17 - desc TE5"]
    #[inline(always)]
    pub fn te5(&mut self) -> Te5W<'_, IcrSpec> {
        Te5W::new(self, 17)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
