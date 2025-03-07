#[doc = "Register `RBR` reader"]
pub struct R(crate::R<RBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RBR` reader - Receive data read port"]
pub type RBR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive data read port"]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receiver Buffer Register (when DLAB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbr](index.html) module"]
pub struct RBR_SPEC;
impl crate::RegisterSpec for RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbr::R](R) reader structure"]
impl crate::Readable for RBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBR to value 0"]
impl crate::Resettable for RBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
