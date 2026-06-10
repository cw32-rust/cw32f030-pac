#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTION` reader - desc ACTION"]
pub type ActionR = crate::BitReader;
#[doc = "Field `ACTION` writer - desc ACTION"]
pub type ActionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SourceR = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VTH` reader - desc VTH"]
pub type VthR = crate::FieldReader;
#[doc = "Field `VTH` writer - desc VTH"]
pub type VthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ACTION"]
    #[inline(always)]
    pub fn action(&self) -> ActionR {
        ActionR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc VTH"]
    #[inline(always)]
    pub fn vth(&self) -> VthR {
        VthR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc ACTION"]
    #[inline(always)]
    pub fn action(&mut self) -> ActionW<'_, Cr0Spec> {
        ActionW::new(self, 1)
    }
    #[doc = "Bits 2:3 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, Cr0Spec> {
        SourceW::new(self, 2)
    }
    #[doc = "Bits 4:7 - desc VTH"]
    #[inline(always)]
    pub fn vth(&mut self) -> VthW<'_, Cr0Spec> {
        VthW::new(self, 4)
    }
    #[doc = "Bit 9 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, Cr0Spec> {
        IeW::new(self, 9)
    }
}
#[doc = "Control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
