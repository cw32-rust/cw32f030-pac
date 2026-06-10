#[doc = "Register `ADDR2` reader"]
pub type R = crate::R<Addr2Spec>;
#[doc = "Register `ADDR2` writer"]
pub type W = crate::W<Addr2Spec>;
#[doc = "Field `ADDR2` reader - desc ADDR2"]
pub type Addr2R = crate::FieldReader;
#[doc = "Field `ADDR2` writer - desc ADDR2"]
pub type Addr2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - desc ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> Addr2R {
        Addr2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - desc ADDR2"]
    #[inline(always)]
    pub fn addr2(&mut self) -> Addr2W<'_, Addr2Spec> {
        Addr2W::new(self, 1)
    }
}
#[doc = "Slave Addrress2\n\nYou can [`read`](crate::Reg::read) this register and get [`addr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr2Spec;
impl crate::RegisterSpec for Addr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr2::R`](R) reader structure"]
impl crate::Readable for Addr2Spec {}
#[doc = "`write(|w| ..)` method takes [`addr2::W`](W) writer structure"]
impl crate::Writable for Addr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR2 to value 0"]
impl crate::Resettable for Addr2Spec {}
