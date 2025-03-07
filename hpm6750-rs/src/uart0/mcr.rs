#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFE` reader - Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS"]
pub type AFE_R = crate::BitReader<bool>;
#[doc = "Field `AFE` writer - Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS"]
pub type AFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `LOOP` reader - Enable loopback mode 0: Disable 1: Enable"]
pub type LOOP_R = crate::BitReader<bool>;
#[doc = "Field `LOOP` writer - Enable loopback mode 0: Disable 1: Enable"]
pub type LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `RTS` reader - Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW"]
pub type RTS_R = crate::BitReader<bool>;
#[doc = "Field `RTS` writer - Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW"]
pub type RTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS"]
    #[inline(always)]
    pub fn afe(&self) -> AFE_R {
        AFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable loopback mode 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS"]
    #[inline(always)]
    pub fn afe(&mut self) -> AFE_W<5> {
        AFE_W::new(self)
    }
    #[doc = "Bit 4 - Enable loopback mode 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W<4> {
        LOOP_W::new(self)
    }
    #[doc = "Bit 1 - Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW"]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W<1> {
        RTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modem Control Register (\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
