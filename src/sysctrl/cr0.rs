#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `SYSCLK` reader - desc SYSCLK"]
pub type SysclkR = crate::FieldReader;
#[doc = "Field `SYSCLK` writer - desc SYSCLK"]
pub type SysclkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PCLKPRS` reader - desc PCLKPRS"]
pub type PclkprsR = crate::FieldReader;
#[doc = "Field `PCLKPRS` writer - desc PCLKPRS"]
pub type PclkprsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HCLKPRS` reader - desc HCLKPRS"]
pub type HclkprsR = crate::FieldReader;
#[doc = "Field `HCLKPRS` writer - desc HCLKPRS"]
pub type HclkprsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - desc SYSCLK"]
    #[inline(always)]
    pub fn sysclk(&self) -> SysclkR {
        SysclkR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - desc PCLKPRS"]
    #[inline(always)]
    pub fn pclkprs(&self) -> PclkprsR {
        PclkprsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - desc HCLKPRS"]
    #[inline(always)]
    pub fn hclkprs(&self) -> HclkprsR {
        HclkprsR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc SYSCLK"]
    #[inline(always)]
    pub fn sysclk(&mut self) -> SysclkW<'_, Cr0Spec> {
        SysclkW::new(self, 0)
    }
    #[doc = "Bits 3:4 - desc PCLKPRS"]
    #[inline(always)]
    pub fn pclkprs(&mut self) -> PclkprsW<'_, Cr0Spec> {
        PclkprsW::new(self, 3)
    }
    #[doc = "Bits 5:7 - desc HCLKPRS"]
    #[inline(always)]
    pub fn hclkprs(&mut self) -> HclkprsW<'_, Cr0Spec> {
        HclkprsW::new(self, 5)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Cr0Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Control Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
