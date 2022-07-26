#[doc = "Register `L3_L4_CFG_0_L3_ADDR_1` reader"]
pub struct R(crate::R<L3_L4_CFG_0_L3_ADDR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L3_L4_CFG_0_L3_ADDR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L3_L4_CFG_0_L3_ADDR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L3_L4_CFG_0_L3_ADDR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L3_L4_CFG_0_L3_ADDR_1` writer"]
pub struct W(crate::W<L3_L4_CFG_0_L3_ADDR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L3_L4_CFG_0_L3_ADDR_1_SPEC>;
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
impl From<crate::W<L3_L4_CFG_0_L3_ADDR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L3_L4_CFG_0_L3_ADDR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3A10` reader - Layer 3 Address 1 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset and Bit 4 (L3DAM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the IP Destination Address field in the IPv4 frames."]
pub type L3A10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `L3A10` writer - Layer 3 Address 1 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset and Bit 4 (L3DAM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the IP Destination Address field in the IPv4 frames."]
pub type L3A10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L3_L4_CFG_0_L3_ADDR_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 1 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset and Bit 4 (L3DAM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the IP Destination Address field in the IPv4 frames."]
    #[inline(always)]
    pub fn l3a10(&self) -> L3A10_R {
        L3A10_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 1 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset and Bit 4 (L3DAM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the IP Destination Address field in the IPv4 frames."]
    #[inline(always)]
    pub fn l3a10(&mut self) -> L3A10_W<0> {
        L3A10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 3 Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l3_l4_cfg_0_l3_addr_1](index.html) module"]
pub struct L3_L4_CFG_0_L3_ADDR_1_SPEC;
impl crate::RegisterSpec for L3_L4_CFG_0_L3_ADDR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l3_l4_cfg_0_l3_addr_1::R](R) reader structure"]
impl crate::Readable for L3_L4_CFG_0_L3_ADDR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l3_l4_cfg_0_l3_addr_1::W](W) writer structure"]
impl crate::Writable for L3_L4_CFG_0_L3_ADDR_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L3_L4_CFG_0_L3_ADDR_1 to value 0"]
impl crate::Resettable for L3_L4_CFG_0_L3_ADDR_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
