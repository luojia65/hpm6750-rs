#[doc = "Register `DMA_RX_DESC_LIST_ADDR` reader"]
pub struct R(crate::R<DMA_RX_DESC_LIST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_DESC_LIST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_DESC_LIST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_DESC_LIST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RX_DESC_LIST_ADDR` writer"]
pub struct W(crate::W<DMA_RX_DESC_LIST_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RX_DESC_LIST_ADDR_SPEC>;
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
impl From<crate::W<DMA_RX_DESC_LIST_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RX_DESC_LIST_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDESLA` reader - Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB bits (1:0, 2:0, or 3:0) for 32-bit, 64-bit, or 128-bit bus width are ignored and internally taken as all-zero by the DMA. Therefore, these LSB bits are read-only (RO)."]
pub type RDESLA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RDESLA` writer - Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB bits (1:0, 2:0, or 3:0) for 32-bit, 64-bit, or 128-bit bus width are ignored and internally taken as all-zero by the DMA. Therefore, these LSB bits are read-only (RO)."]
pub type RDESLA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_RX_DESC_LIST_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB bits (1:0, 2:0, or 3:0) for 32-bit, 64-bit, or 128-bit bus width are ignored and internally taken as all-zero by the DMA. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB bits (1:0, 2:0, or 3:0) for 32-bit, 64-bit, or 128-bit bus width are ignored and internally taken as all-zero by the DMA. Therefore, these LSB bits are read-only (RO)."]
    #[inline(always)]
    pub fn rdesla(&mut self) -> RDESLA_W<0> {
        RDESLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Descriptor List Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_desc_list_addr](index.html) module"]
pub struct DMA_RX_DESC_LIST_ADDR_SPEC;
impl crate::RegisterSpec for DMA_RX_DESC_LIST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_desc_list_addr::R](R) reader structure"]
impl crate::Readable for DMA_RX_DESC_LIST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rx_desc_list_addr::W](W) writer structure"]
impl crate::Writable for DMA_RX_DESC_LIST_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_RX_DESC_LIST_ADDR to value 0"]
impl crate::Resettable for DMA_RX_DESC_LIST_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
