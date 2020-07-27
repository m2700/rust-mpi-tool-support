use std::{
    ops::{Deref, DerefMut},
    os::raw::{c_double, c_float},
};

mod c_num {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(env!("C_NUM_BINDINGS"));
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Debug, Hash, Default)]
pub struct Complex<T>(c_num::__BindgenComplex<T>);
impl<T: Eq> Eq for Complex<T> {}
impl<T> From<c_num::__BindgenComplex<T>> for Complex<T> {
    #[inline]
    fn from(src: c_num::__BindgenComplex<T>) -> Self {
        Self(src)
    }
}
impl<T> From<Complex<T>> for c_num::__BindgenComplex<T> {
    #[inline]
    fn from(src: Complex<T>) -> Self {
        src.0
    }
}
impl<T> Deref for Complex<T> {
    type Target = c_num::__BindgenComplex<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Complex<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> Complex<T> {
    #[inline]
    pub const fn new(real: T, imag: T) -> Self {
        Self(c_num::__BindgenComplex { re: real, im: imag })
    }
}
impl Complex<c_float> {
    #[inline]
    pub fn real(self) -> c_float {
        unsafe { c_num::crealf(*self) }
    }
    #[inline]
    pub fn imag(self) -> c_float {
        unsafe { c_num::cimagf(*self) }
    }

    #[inline]
    pub fn acos(self) -> Self {
        unsafe { c_num::cacosf(*self) }.into()
    }
    #[inline]
    pub fn asin(self) -> Self {
        unsafe { c_num::casinf(*self) }.into()
    }
    #[inline]
    pub fn atan(self) -> Self {
        unsafe { c_num::catanf(*self) }.into()
    }

    #[inline]
    pub fn cos(self) -> Self {
        unsafe { c_num::ccosf(*self) }.into()
    }
    #[inline]
    pub fn sin(self) -> Self {
        unsafe { c_num::csinf(*self) }.into()
    }
    #[inline]
    pub fn tan(self) -> Self {
        unsafe { c_num::ctanf(*self) }.into()
    }

    #[inline]
    pub fn acosh(self) -> Self {
        unsafe { c_num::cacoshf(*self) }.into()
    }
    #[inline]
    pub fn asinh(self) -> Self {
        unsafe { c_num::casinhf(*self) }.into()
    }
    #[inline]
    pub fn atanh(self) -> Self {
        unsafe { c_num::catanhf(*self) }.into()
    }

    #[inline]
    pub fn cosh(self) -> Self {
        unsafe { c_num::ccoshf(*self) }.into()
    }
    #[inline]
    pub fn sinh(self) -> Self {
        unsafe { c_num::csinhf(*self) }.into()
    }
    #[inline]
    pub fn tanh(self) -> Self {
        unsafe { c_num::ctanhf(*self) }.into()
    }

    #[inline]
    pub fn exp(self) -> Self {
        unsafe { c_num::cexpf(*self) }.into()
    }
    #[inline]
    pub fn log(self) -> Self {
        unsafe { c_num::clogf(*self) }.into()
    }

    #[inline]
    pub fn abs(self) -> c_float {
        unsafe { c_num::cabsf(*self) }
    }

    #[inline]
    pub fn pow(self, exp: Self) -> Self {
        unsafe { c_num::cpowf(*self, *exp) }.into()
    }
    #[inline]
    pub fn sqrt(self) -> Self {
        unsafe { c_num::csqrtf(*self) }.into()
    }

    #[inline]
    pub fn arg(self) -> c_float {
        unsafe { c_num::cargf(*self) }
    }

    #[inline]
    pub fn conj(self) -> Self {
        unsafe { c_num::conjf(*self) }.into()
    }

    #[inline]
    pub fn proj(self) -> Self {
        unsafe { c_num::cprojf(*self) }.into()
    }
}

impl Complex<c_double> {
    #[inline]
    pub fn real(self) -> c_double {
        unsafe { c_num::creal(*self) }
    }
    #[inline]
    pub fn imag(self) -> c_double {
        unsafe { c_num::cimag(*self) }
    }

    #[inline]
    pub fn acos(self) -> Self {
        unsafe { c_num::cacos(*self) }.into()
    }
    #[inline]
    pub fn asin(self) -> Self {
        unsafe { c_num::casin(*self) }.into()
    }
    #[inline]
    pub fn atan(self) -> Self {
        unsafe { c_num::catan(*self) }.into()
    }

    #[inline]
    pub fn cos(self) -> Self {
        unsafe { c_num::ccos(*self) }.into()
    }
    #[inline]
    pub fn sin(self) -> Self {
        unsafe { c_num::csin(*self) }.into()
    }
    #[inline]
    pub fn tan(self) -> Self {
        unsafe { c_num::ctan(*self) }.into()
    }

    #[inline]
    pub fn acosh(self) -> Self {
        unsafe { c_num::cacosh(*self) }.into()
    }
    #[inline]
    pub fn asinh(self) -> Self {
        unsafe { c_num::casinh(*self) }.into()
    }
    #[inline]
    pub fn atanh(self) -> Self {
        unsafe { c_num::catanh(*self) }.into()
    }

    #[inline]
    pub fn cosh(self) -> Self {
        unsafe { c_num::ccosh(*self) }.into()
    }
    #[inline]
    pub fn sinh(self) -> Self {
        unsafe { c_num::csinh(*self) }.into()
    }
    #[inline]
    pub fn tanh(self) -> Self {
        unsafe { c_num::ctanh(*self) }.into()
    }

    #[inline]
    pub fn exp(self) -> Self {
        unsafe { c_num::cexp(*self) }.into()
    }
    #[inline]
    pub fn log(self) -> Self {
        unsafe { c_num::clog(*self) }.into()
    }

    #[inline]
    pub fn abs(self) -> c_double {
        unsafe { c_num::cabs(*self) }
    }

    #[inline]
    pub fn pow(self, exp: Self) -> Self {
        unsafe { c_num::cpow(*self, *exp) }.into()
    }
    #[inline]
    pub fn sqrt(self) -> Self {
        unsafe { c_num::csqrt(*self) }.into()
    }

    #[inline]
    pub fn arg(self) -> c_double {
        unsafe { c_num::carg(*self) }
    }

    #[inline]
    pub fn conj(self) -> Self {
        unsafe { c_num::conj(*self) }.into()
    }

    #[inline]
    pub fn proj(self) -> Self {
        unsafe { c_num::cproj(*self) }.into()
    }
}
