#[doc = "Reader of register CH1DATA"]
pub type R = crate::R<u32, super::CH1DATA>;
#[doc = "Writer for register CH1DATA"]
pub type W = crate::W<u32, super::CH1DATA>;
#[doc = "Register CH1DATA `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::CH1DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0x0800 }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Channel 1 Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R { DATA_R::new((self.bits & 0x0fff) as u16) }
}
impl W {
    #[doc = "Bits 0:11 - Channel 1 Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W { DATA_W { w: self } }
}
