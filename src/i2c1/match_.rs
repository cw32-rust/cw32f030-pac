#[doc = "Register `MATCH` reader"]
pub type R = crate::R<MatchSpec>;
#[doc = "Field `ADDR0` reader - desc ADDR0"]
pub type Addr0R = crate::BitReader;
#[doc = "Field `ADDR1` reader - desc ADDR1"]
pub type Addr1R = crate::BitReader;
#[doc = "Field `ADDR2` reader - desc ADDR2"]
pub type Addr2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> Addr0R {
        Addr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ADDR1"]
    #[inline(always)]
    pub fn addr1(&self) -> Addr1R {
        Addr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> Addr2R {
        Addr2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Slave Addrress match flag\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatchSpec;
impl crate::RegisterSpec for MatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`match_::R`](R) reader structure"]
impl crate::Readable for MatchSpec {}
#[doc = "`reset()` method sets MATCH to value 0"]
impl crate::Resettable for MatchSpec {}
