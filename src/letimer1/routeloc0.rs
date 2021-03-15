#[doc = "Reader of register ROUTELOC0"]
pub type R = crate::R<u32, super::ROUTELOC0>;
#[doc = "Writer for register ROUTELOC0"]
pub type W = crate::W<u32, super::ROUTELOC0>;
#[doc = "Register ROUTELOC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT0LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
}
impl From<OUT0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT0LOC_A) -> Self { variant as _ }
}
#[doc = "Reader of field `OUT0LOC`"]
pub type OUT0LOC_R = crate::R<u8, OUT0LOC_A>;
impl OUT0LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OUT0LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OUT0LOC_A::LOC0),
            1 => Val(OUT0LOC_A::LOC1),
            2 => Val(OUT0LOC_A::LOC2),
            3 => Val(OUT0LOC_A::LOC3),
            4 => Val(OUT0LOC_A::LOC4),
            5 => Val(OUT0LOC_A::LOC5),
            6 => Val(OUT0LOC_A::LOC6),
            7 => Val(OUT0LOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool { *self == OUT0LOC_A::LOC0 }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool { *self == OUT0LOC_A::LOC1 }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool { *self == OUT0LOC_A::LOC2 }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool { *self == OUT0LOC_A::LOC3 }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool { *self == OUT0LOC_A::LOC4 }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool { *self == OUT0LOC_A::LOC5 }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool { *self == OUT0LOC_A::LOC6 }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool { *self == OUT0LOC_A::LOC7 }
}
#[doc = "Write proxy for field `OUT0LOC`"]
pub struct OUT0LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT0LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT0LOC_A) -> &'a mut W { unsafe { self.bits(variant.into()) } }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W { self.variant(OUT0LOC_A::LOC0) }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W { self.variant(OUT0LOC_A::LOC1) }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W { self.variant(OUT0LOC_A::LOC2) }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W { self.variant(OUT0LOC_A::LOC3) }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W { self.variant(OUT0LOC_A::LOC4) }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W { self.variant(OUT0LOC_A::LOC5) }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W { self.variant(OUT0LOC_A::LOC6) }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W { self.variant(OUT0LOC_A::LOC7) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT1LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
}
impl From<OUT1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT1LOC_A) -> Self { variant as _ }
}
#[doc = "Reader of field `OUT1LOC`"]
pub type OUT1LOC_R = crate::R<u8, OUT1LOC_A>;
impl OUT1LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OUT1LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OUT1LOC_A::LOC0),
            1 => Val(OUT1LOC_A::LOC1),
            2 => Val(OUT1LOC_A::LOC2),
            3 => Val(OUT1LOC_A::LOC3),
            4 => Val(OUT1LOC_A::LOC4),
            5 => Val(OUT1LOC_A::LOC5),
            6 => Val(OUT1LOC_A::LOC6),
            7 => Val(OUT1LOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool { *self == OUT1LOC_A::LOC0 }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool { *self == OUT1LOC_A::LOC1 }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool { *self == OUT1LOC_A::LOC2 }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool { *self == OUT1LOC_A::LOC3 }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool { *self == OUT1LOC_A::LOC4 }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool { *self == OUT1LOC_A::LOC5 }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool { *self == OUT1LOC_A::LOC6 }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool { *self == OUT1LOC_A::LOC7 }
}
#[doc = "Write proxy for field `OUT1LOC`"]
pub struct OUT1LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT1LOC_A) -> &'a mut W { unsafe { self.bits(variant.into()) } }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W { self.variant(OUT1LOC_A::LOC0) }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W { self.variant(OUT1LOC_A::LOC1) }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W { self.variant(OUT1LOC_A::LOC2) }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W { self.variant(OUT1LOC_A::LOC3) }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W { self.variant(OUT1LOC_A::LOC4) }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W { self.variant(OUT1LOC_A::LOC5) }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W { self.variant(OUT1LOC_A::LOC6) }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W { self.variant(OUT1LOC_A::LOC7) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn out0loc(&self) -> OUT0LOC_R { OUT0LOC_R::new((self.bits & 0x3f) as u8) }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn out1loc(&self) -> OUT1LOC_R { OUT1LOC_R::new(((self.bits >> 8) & 0x3f) as u8) }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn out0loc(&mut self) -> OUT0LOC_W { OUT0LOC_W { w: self } }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn out1loc(&mut self) -> OUT1LOC_W { OUT1LOC_W { w: self } }
}
