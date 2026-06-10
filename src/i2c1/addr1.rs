#[doc = "Register `ADDR1` reader"]
pub type R = crate::R<Addr1Spec>;
#[doc = "Register `ADDR1` writer"]
pub type W = crate::W<Addr1Spec>;
#[doc = "Field `ADDR1` reader - desc ADDR1"]
pub type Addr1R = crate::FieldReader;
#[doc = "Field `ADDR1` writer - desc ADDR1"]
pub type Addr1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - desc ADDR1"]
    #[inline(always)]
    pub fn addr1(&self) -> Addr1R {
        Addr1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - desc ADDR1"]
    #[inline(always)]
    pub fn addr1(&mut self) -> Addr1W<'_, Addr1Spec> {
        Addr1W::new(self, 1)
    }
}
#[doc = "Slave Addrress1\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr1Spec;
impl crate::RegisterSpec for Addr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr1::R`](R) reader structure"]
impl crate::Readable for Addr1Spec {}
#[doc = "`write(|w| ..)` method takes [`addr1::W`](W) writer structure"]
impl crate::Writable for Addr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::Resettable for Addr1Spec {}
