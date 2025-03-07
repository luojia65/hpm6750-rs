#[doc = "Register `HISTOGRAM_FIFO_DATA129` reader"]
pub struct R(crate::R<HISTOGRAM_FIFO_DATA129_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HISTOGRAM_FIFO_DATA129_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HISTOGRAM_FIFO_DATA129_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HISTOGRAM_FIFO_DATA129_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HIST_Y` reader - the appearance of bin x (x=(address-DATA0)/4)"]
pub type HIST_Y_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - the appearance of bin x (x=(address-DATA0)/4)"]
    #[inline(always)]
    pub fn hist_y(&self) -> HIST_Y_R {
        HIST_Y_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Histogram Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [histogram_fifo_data129](index.html) module"]
pub struct HISTOGRAM_FIFO_DATA129_SPEC;
impl crate::RegisterSpec for HISTOGRAM_FIFO_DATA129_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [histogram_fifo_data129::R](R) reader structure"]
impl crate::Readable for HISTOGRAM_FIFO_DATA129_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HISTOGRAM_FIFO_DATA129 to value 0"]
impl crate::Resettable for HISTOGRAM_FIFO_DATA129_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
