#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CH1FLT` reader - desc CH1FLT"]
pub type Ch1fltR = crate::FieldReader;
#[doc = "Field `CH1FLT` writer - desc CH1FLT"]
pub type Ch1fltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1POL` reader - desc CH1POL"]
pub type Ch1polR = crate::BitReader;
#[doc = "Field `CH1POL` writer - desc CH1POL"]
pub type Ch1polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2FLT` reader - desc CH2FLT"]
pub type Ch2fltR = crate::FieldReader;
#[doc = "Field `CH2FLT` writer - desc CH2FLT"]
pub type Ch2fltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2POL` reader - desc CH2POL"]
pub type Ch2polR = crate::BitReader;
#[doc = "Field `CH2POL` writer - desc CH2POL"]
pub type Ch2polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3FLT` reader - desc CH3FLT"]
pub type Ch3fltR = crate::FieldReader;
#[doc = "Field `CH3FLT` writer - desc CH3FLT"]
pub type Ch3fltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH3POL` reader - desc CH3POL"]
pub type Ch3polR = crate::BitReader;
#[doc = "Field `CH3POL` writer - desc CH3POL"]
pub type Ch3polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4FLT` reader - desc CH4FLT"]
pub type Ch4fltR = crate::FieldReader;
#[doc = "Field `CH4FLT` writer - desc CH4FLT"]
pub type Ch4fltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH4POL` reader - desc CH4POL"]
pub type Ch4polR = crate::BitReader;
#[doc = "Field `CH4POL` writer - desc CH4POL"]
pub type Ch4polW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc CH1FLT"]
    #[inline(always)]
    pub fn ch1flt(&self) -> Ch1fltR {
        Ch1fltR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc CH1POL"]
    #[inline(always)]
    pub fn ch1pol(&self) -> Ch1polR {
        Ch1polR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc CH2FLT"]
    #[inline(always)]
    pub fn ch2flt(&self) -> Ch2fltR {
        Ch2fltR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc CH2POL"]
    #[inline(always)]
    pub fn ch2pol(&self) -> Ch2polR {
        Ch2polR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc CH3FLT"]
    #[inline(always)]
    pub fn ch3flt(&self) -> Ch3fltR {
        Ch3fltR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc CH3POL"]
    #[inline(always)]
    pub fn ch3pol(&self) -> Ch3polR {
        Ch3polR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc CH4FLT"]
    #[inline(always)]
    pub fn ch4flt(&self) -> Ch4fltR {
        Ch4fltR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc CH4POL"]
    #[inline(always)]
    pub fn ch4pol(&self) -> Ch4polR {
        Ch4polR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc CH1FLT"]
    #[inline(always)]
    pub fn ch1flt(&mut self) -> Ch1fltW<'_, Cr1Spec> {
        Ch1fltW::new(self, 0)
    }
    #[doc = "Bit 3 - desc CH1POL"]
    #[inline(always)]
    pub fn ch1pol(&mut self) -> Ch1polW<'_, Cr1Spec> {
        Ch1polW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc CH2FLT"]
    #[inline(always)]
    pub fn ch2flt(&mut self) -> Ch2fltW<'_, Cr1Spec> {
        Ch2fltW::new(self, 4)
    }
    #[doc = "Bit 7 - desc CH2POL"]
    #[inline(always)]
    pub fn ch2pol(&mut self) -> Ch2polW<'_, Cr1Spec> {
        Ch2polW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc CH3FLT"]
    #[inline(always)]
    pub fn ch3flt(&mut self) -> Ch3fltW<'_, Cr1Spec> {
        Ch3fltW::new(self, 8)
    }
    #[doc = "Bit 11 - desc CH3POL"]
    #[inline(always)]
    pub fn ch3pol(&mut self) -> Ch3polW<'_, Cr1Spec> {
        Ch3polW::new(self, 11)
    }
    #[doc = "Bits 12:14 - desc CH4FLT"]
    #[inline(always)]
    pub fn ch4flt(&mut self) -> Ch4fltW<'_, Cr1Spec> {
        Ch4fltW::new(self, 12)
    }
    #[doc = "Bit 15 - desc CH4POL"]
    #[inline(always)]
    pub fn ch4pol(&mut self) -> Ch4polW<'_, Cr1Spec> {
        Ch4polW::new(self, 15)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
