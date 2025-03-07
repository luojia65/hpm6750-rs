#[doc = "Register `AFFILIATE_CPU1_SET` reader"]
pub struct R(crate::R<AFFILIATE_CPU1_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFFILIATE_CPU1_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFFILIATE_CPU1_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFFILIATE_CPU1_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFFILIATE_CPU1_SET` writer"]
pub struct W(crate::W<AFFILIATE_CPU1_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFFILIATE_CPU1_SET_SPEC>;
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
impl From<crate::W<AFFILIATE_CPU1_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFFILIATE_CPU1_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK` reader - Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
pub type LINK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINK` writer - Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
pub type LINK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AFFILIATE_CPU1_SET_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    #[inline(always)]
    pub fn link(&mut self) -> LINK_W<0> {
        LINK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Affiliate of Group\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [affiliate_cpu1_set](index.html) module"]
pub struct AFFILIATE_CPU1_SET_SPEC;
impl crate::RegisterSpec for AFFILIATE_CPU1_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [affiliate_cpu1_set::R](R) reader structure"]
impl crate::Readable for AFFILIATE_CPU1_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [affiliate_cpu1_set::W](W) writer structure"]
impl crate::Writable for AFFILIATE_CPU1_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFFILIATE_CPU1_SET to value 0x01"]
impl crate::Resettable for AFFILIATE_CPU1_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
