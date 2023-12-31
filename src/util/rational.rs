use std::cmp::Ordering;
use std::mem::MaybeUninit;
use std::ops::{Add, Div, Mul, Sub};
use std::{fmt, ptr};

use avutil_wasmedge;

#[derive(Copy, Clone)]
pub struct Rational(pub i32, pub i32);

impl Rational {
    #[inline]
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Rational(numerator, denominator)
    }

    #[inline]
    pub fn numerator(&self) -> i32 {
        self.0
    }

    #[inline]
    pub fn denominator(&self) -> i32 {
        self.1
    }

    #[inline]
    pub fn reduce(&self) -> Rational {
        match self.reduce_with_limit(i32::max_value()) {
            Ok(r) => r,
            Err(r) => r,
        }
    }

    #[inline]
    pub fn reduce_with_limit(&self, max: i32) -> Result<Rational, Rational> {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();

            let exact = avutil_wasmedge::av_reduce(
                num.as_ptr() as u32,
                den.as_ptr() as u32,
                i64::from(self.numerator()),
                i64::from(self.denominator()),
                i64::from(max),
            );

            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());
            if exact == 1 {
                Ok(Rational(num, den))
            } else {
                Err(Rational(num, den))
            }
        }
    }

    #[inline]
    pub fn invert(&self) -> Rational {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avutil_wasmedge::av_inv_q(
                self.numerator(),
                self.denominator(),
                num.as_ptr() as u32,
                den.as_ptr() as u32,
            );
            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());
            Rational::new(num, den)
        }
    }
}

impl From<f64> for Rational {
    #[inline]
    fn from(value: f64) -> Rational {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avutil_wasmedge::av_d2q(value, i32::MAX, num.as_ptr() as u32, den.as_ptr() as u32);
            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());
            Rational::new(num, den)
        }
    }
}

impl From<Rational> for f64 {
    #[inline]
    fn from(value: Rational) -> f64 {
        unsafe { avutil_wasmedge::av_q2d(value.numerator(), value.denominator()) }
    }
}

impl From<Rational> for u32 {
    #[inline]
    fn from(value: Rational) -> u32 {
        unsafe { avutil_wasmedge::av_q2intfloat(value.numerator(), value.denominator()) }
    }
}

impl From<(i32, i32)> for Rational {
    fn from((num, den): (i32, i32)) -> Rational {
        Rational::new(num, den)
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        if self.0 == other.0 && self.1 == other.1 {
            return true;
        }

        let a = self.reduce();
        let b = other.reduce();

        if a.0 == b.0 && a.1 == b.1 {
            return true;
        }

        false
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            match avutil_wasmedge::av_cmp_q(
                self.numerator(),
                self.denominator(),
                other.numerator(),
                other.denominator(),
            ) {
                0 => Some(Ordering::Equal),
                1 => Some(Ordering::Greater),
                -1 => Some(Ordering::Less),

                _ => None,
            }
        }
    }
}

impl Add for Rational {
    type Output = Rational;

    #[inline]
    fn add(self, other: Rational) -> Rational {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avutil_wasmedge::av_add_q(
                self.numerator(),
                self.denominator(),
                other.numerator(),
                other.denominator(),
                num.as_ptr() as u32,
                den.as_ptr() as u32,
            );
            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());
            Rational::new(num, den)
        }
    }
}

impl Sub for Rational {
    type Output = Rational;

    #[inline]
    fn sub(self, other: Rational) -> Rational {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avutil_wasmedge::av_sub_q(
                self.numerator(),
                self.denominator(),
                other.numerator(),
                other.denominator(),
                num.as_ptr() as u32,
                den.as_ptr() as u32,
            );
            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());
            Rational::new(num, den)
        }
    }
}

impl Mul for Rational {
    type Output = Rational;

    #[inline]
    fn mul(self, other: Rational) -> Rational {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avutil_wasmedge::av_mul_q(
                self.numerator(),
                self.denominator(),
                other.numerator(),
                other.denominator(),
                num.as_ptr() as u32,
                den.as_ptr() as u32,
            );

            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());
            Rational::new(num, den)
        }
    }
}

impl Div for Rational {
    type Output = Rational;

    #[inline]
    fn div(self, other: Rational) -> Rational {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avutil_wasmedge::av_div_q(
                self.numerator(),
                self.denominator(),
                other.numerator(),
                other.denominator(),
                num.as_ptr() as u32,
                den.as_ptr() as u32,
            );

            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());
            Rational::new(num, den)
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!("{}/{}", self.numerator(), self.denominator()))
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!(
            "Rational({}/{})",
            self.numerator(),
            self.denominator()
        ))
    }
}

#[inline]
pub fn nearer(q: Rational, q1: Rational, q2: Rational) -> Ordering {
    unsafe {
        match avutil_wasmedge::av_nearer_q(
            q.numerator(),
            q.denominator(),
            q1.numerator(),
            q1.denominator(),
            q2.numerator(),
            q2.denominator(),
        ) {
            1 => Ordering::Greater,
            -1 => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}
