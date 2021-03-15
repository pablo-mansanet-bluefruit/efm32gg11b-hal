#[doc = "Reader of register QDATA1"]
pub type R = crate::R<u32, super::QDATA1>;
#[doc = "Writer for register QDATA1"]
pub type W = crate::W<u32, super::QDATA1>;
#[doc = "Register QDATA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::QDATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `QDATA1`"]
pub type QDATA1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QDATA1`"]
pub struct QDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> QDATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Access"]
    #[inline(always)]
    pub fn qdata1(&self) -> QDATA1_R { QDATA1_R::new((self.bits & 0xffff_ffff) as u32) }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Access"]
    #[inline(always)]
    pub fn qdata1(&mut self) -> QDATA1_W { QDATA1_W { w: self } }
}
