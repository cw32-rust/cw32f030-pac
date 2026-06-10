#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `SWDIO` reader - desc SWDIO"]
pub type SwdioR = crate::BitReader;
#[doc = "Field `SWDIO` writer - desc SWDIO"]
pub type SwdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUP` reader - desc LOCKUP"]
pub type LockupR = crate::BitReader;
#[doc = "Field `LOCKUP` writer - desc LOCKUP"]
pub type LockupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPCLK` reader - desc WAKEUPCLK"]
pub type WakeupclkR = crate::BitReader;
#[doc = "Field `WAKEUPCLK` writer - desc WAKEUPCLK"]
pub type WakeupclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - desc SWDIO"]
    #[inline(always)]
    pub fn swdio(&self) -> SwdioR {
        SwdioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WAKEUPCLK"]
    #[inline(always)]
    pub fn wakeupclk(&self) -> WakeupclkR {
        WakeupclkR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc SWDIO"]
    #[inline(always)]
    pub fn swdio(&mut self) -> SwdioW<'_, Cr2Spec> {
        SwdioW::new(self, 1)
    }
    #[doc = "Bit 2 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LockupW<'_, Cr2Spec> {
        LockupW::new(self, 2)
    }
    #[doc = "Bit 3 - desc WAKEUPCLK"]
    #[inline(always)]
    pub fn wakeupclk(&mut self) -> WakeupclkW<'_, Cr2Spec> {
        WakeupclkW::new(self, 3)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Cr2Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
