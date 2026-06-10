#[doc = "Register `ADDR0` reader"]
pub type R = crate::R<Addr0Spec>;
#[doc = "Register `ADDR0` writer"]
pub type W = crate::W<Addr0Spec>;
#[doc = "Field `GC` reader - desc GC"]
pub type GcR = crate::BitReader;
#[doc = "Field `GC` writer - desc GC"]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR0` reader - desc ADDR0"]
pub type Addr0R = crate::FieldReader;
#[doc = "Field `ADDR0` writer - desc ADDR0"]
pub type Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - desc GC"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - desc ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> Addr0R {
        Addr0R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc GC"]
    #[inline(always)]
    pub fn gc(&mut self) -> GcW<'_, Addr0Spec> {
        GcW::new(self, 0)
    }
    #[doc = "Bits 1:7 - desc ADDR0"]
    #[inline(always)]
    pub fn addr0(&mut self) -> Addr0W<'_, Addr0Spec> {
        Addr0W::new(self, 1)
    }
}
#[doc = "Slave Addrress0\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr0Spec;
impl crate::RegisterSpec for Addr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr0::R`](R) reader structure"]
impl crate::Readable for Addr0Spec {}
#[doc = "`write(|w| ..)` method takes [`addr0::W`](W) writer structure"]
impl crate::Writable for Addr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR0 to value 0"]
impl crate::Resettable for Addr0Spec {}
