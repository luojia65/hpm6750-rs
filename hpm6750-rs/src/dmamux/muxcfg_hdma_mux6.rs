#[doc = "Register `MUXCFG_HDMA_MUX6` reader"]
pub struct R(crate::R<MUXCFG_HDMA_MUX6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUXCFG_HDMA_MUX6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUXCFG_HDMA_MUX6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUXCFG_HDMA_MUX6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUXCFG_HDMA_MUX6` writer"]
pub struct W(crate::W<MUXCFG_HDMA_MUX6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUXCFG_HDMA_MUX6_SPEC>;
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
impl From<crate::W<MUXCFG_HDMA_MUX6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUXCFG_HDMA_MUX6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUXCFG_HDMA_MUX6_SPEC, bool, O>;
#[doc = "Field `SOURCE` reader - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the \"DMA MUX Mapping\""]
pub type SOURCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOURCE` writer - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the \"DMA MUX Mapping\""]
pub type SOURCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MUXCFG_HDMA_MUX6_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 31 - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 0:6 - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the \"DMA MUX Mapping\""]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 0:6 - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the \"DMA MUX Mapping\""]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W<0> {
        SOURCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HDMA MUX6 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxcfg_hdma_mux6](index.html) module"]
pub struct MUXCFG_HDMA_MUX6_SPEC;
impl crate::RegisterSpec for MUXCFG_HDMA_MUX6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [muxcfg_hdma_mux6::R](R) reader structure"]
impl crate::Readable for MUXCFG_HDMA_MUX6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [muxcfg_hdma_mux6::W](W) writer structure"]
impl crate::Writable for MUXCFG_HDMA_MUX6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUXCFG_HDMA_MUX6 to value 0"]
impl crate::Resettable for MUXCFG_HDMA_MUX6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
