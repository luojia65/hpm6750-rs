#[doc = "Register `R2SK_FO2S7` reader"]
pub struct R(crate::R<R2SK_FO2S7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R2SK_FO2S7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R2SK_FO2S7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R2SK_FO2S7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FO2S0` reader - FIFO out to KMAN, will be SDP engine key."]
pub type FO2S0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO out to KMAN, will be SDP engine key."]
    #[inline(always)]
    pub fn fo2s0(&self) -> FO2S0_R {
        FO2S0_R::new(self.bits)
    }
}
#[doc = "FIFO out to SDP as AES engine key\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2sk_fo2s7](index.html) module"]
pub struct R2SK_FO2S7_SPEC;
impl crate::RegisterSpec for R2SK_FO2S7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r2sk_fo2s7::R](R) reader structure"]
impl crate::Readable for R2SK_FO2S7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R2SK_FO2S7 to value 0"]
impl crate::Resettable for R2SK_FO2S7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
