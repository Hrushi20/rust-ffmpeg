use avutil_wasmedge;
use {Rational, Rounding};

pub const TIME_BASE: Rational = Rational(1, 1000000);

pub trait Rescale {
    fn rescale<S, D>(&self, source: S, destination: D) -> i64
    where
        S: Into<Rational>,
        D: Into<Rational>;

    fn rescale_with<S, D>(&self, source: S, destination: D, rounding: Rounding) -> i64
    where
        S: Into<Rational>,
        D: Into<Rational>;
}

impl<T: Into<i64> + Clone> Rescale for T {
    fn rescale<S, D>(&self, source: S, destination: D) -> i64
    where
        S: Into<Rational>,
        D: Into<Rational>,
    {
        let src_rational = source.into();
        let dest_rational = destination.into();

        unsafe {
            avutil_wasmedge::av_rescale_q(
                self.clone().into(),
                src_rational.numerator(),
                src_rational.denominator(),
                dest_rational.numerator(),
                dest_rational.denominator(),
            )
        }
    }

    fn rescale_with<S, D>(&self, source: S, destination: D, rounding: Rounding) -> i64
    where
        S: Into<Rational>,
        D: Into<Rational>,
    {
        unsafe {
            let src_rational = source.into();
            let dest_rational = destination.into();

            avutil_wasmedge::av_rescale_q_rnd(
                self.clone().into(),
                src_rational.numerator(),
                src_rational.denominator(),
                dest_rational.numerator(),
                dest_rational.denominator(),
                rounding.into(),
            )
        }
    }
}
