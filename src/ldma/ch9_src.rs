#[doc = "Reader of register CH9_SRC"]
pub type R = crate::R<u32, super::CH9_SRC>;
#[doc = "Writer for register CH9_SRC"]
pub type W = crate::W<u32, super::CH9_SRC>;
#[doc = "Register CH9_SRC `reset()`'s with value 0"]
impl crate::ResetValue for super::CH9_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `SRCADDR`"]
pub type SRCADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SRCADDR`"]
pub struct SRCADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SRCADDR_R { SRCADDR_R::new((self.bits & 0xffff_ffff) as u32) }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&mut self) -> SRCADDR_W { SRCADDR_W { w: self } }
}
