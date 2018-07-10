use super::*;
// -- BitXorAssign

impl BitXorAssign for I32x4 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXorAssign for I32x4_41 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXorAssign for I32x8 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}
impl BitXorAssign for I64x2 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I64x2) {
        *self = I64x2(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXorAssign for I64x2_41 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXorAssign for I64x4 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I64x4) {
        *self = I64x4(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}
impl BitXorAssign for F32x4 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_xor_ps(self.0, rhs.0) })
    }
}
impl BitXorAssign for F32x8 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_xor_ps(self.0, rhs.0) })
    }
}

impl BitXorAssign for F64x2 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_xor_pd(self.0, rhs.0) })
    }
}
impl BitXorAssign for F64x4 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_xor_pd(self.0, rhs.0) })
    }
}
