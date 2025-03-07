#[doc = "Register `DMA_CURR_HOST_TX_BUF` reader"]
pub struct R(crate::R<DMA_CURR_HOST_TX_BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CURR_HOST_TX_BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CURR_HOST_TX_BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CURR_HOST_TX_BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CURR_HOST_TX_BUF` writer"]
pub struct W(crate::W<DMA_CURR_HOST_TX_BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CURR_HOST_TX_BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_CURR_HOST_TX_BUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CURR_HOST_TX_BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURTBUFAPTR` reader - Host Transmit Buffer Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CURTBUFAPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURTBUFAPTR` writer - Host Transmit Buffer Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CURTBUFAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CURR_HOST_TX_BUF_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtbufaptr(&mut self) -> CURTBUFAPTR_W<0> {
        CURTBUFAPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Host Transmit Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_curr_host_tx_buf](index.html) module"]
pub struct DMA_CURR_HOST_TX_BUF_SPEC;
impl crate::RegisterSpec for DMA_CURR_HOST_TX_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_curr_host_tx_buf::R](R) reader structure"]
impl crate::Readable for DMA_CURR_HOST_TX_BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_curr_host_tx_buf::W](W) writer structure"]
impl crate::Writable for DMA_CURR_HOST_TX_BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CURR_HOST_TX_BUF to value 0"]
impl crate::Resettable for DMA_CURR_HOST_TX_BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
