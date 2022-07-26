#[doc = "Register `CAPPOS_17` reader"]
pub struct R(crate::R<CAPPOS_17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPPOS_17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPPOS_17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPPOS_17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPPOS` reader - counter value captured at input posedge"]
pub type CAPPOS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 4:31 - counter value captured at input posedge"]
    #[inline(always)]
    pub fn cappos(&self) -> CAPPOS_R {
        CAPPOS_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
#[doc = "Capture rising edge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cappos_17](index.html) module"]
pub struct CAPPOS_17_SPEC;
impl crate::RegisterSpec for CAPPOS_17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cappos_17::R](R) reader structure"]
impl crate::Readable for CAPPOS_17_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPPOS_17 to value 0"]
impl crate::Resettable for CAPPOS_17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
