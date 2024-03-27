use {crate::algebra::*, core::iter::Iterator};

pub trait Distributive<T = Self> {}

pub trait Divisibility: Sized {
    ///Determines if there exists an element `x` such that `self*x` = `rhs`
    fn divides(self, rhs: Self) -> bool;

    ///Finds an element `x` such that `self*x` = `rhs` if such an element exists
    fn divide(self, rhs: Self) -> Option<Self>;

    ///Determines if this element has a multiplicative inverse
    fn unit(&self) -> bool;

    ///Finds this element's multiplicative inverse if it exists
    fn inverse(self) -> Option<Self>;
}

pub trait Primality: Divisibility {
    fn irreducible(&self) -> bool;

    fn prime(&self) -> bool;
}

pub trait NoZeroDivisors {}

///A trait for finding the greatest common divisor and least common multiple of two elements
pub trait GCD: Sized {
    fn gcd(self, rhs: Self) -> Self;
    fn lcm(self, rhs: Self) -> Self;
}

///A trait for finding the Bezout coefficients of a pair of elements
pub trait Bezout: GCD {
    #[inline]
    fn bezout_coefficients(self, rhs: Self) -> (Self, Self) {
        let (a, b, _) = self.bezout_with_gcd(rhs);
        (a, b)
    }

    fn bezout_with_gcd(self, rhs: Self) -> (Self, Self, Self);
}

pub trait UniquelyFactorizable: Sized {}

pub trait Factorizable: Sized {
    type Factors: Iterator<Item = Self>;
    fn factors(self) -> Self::Factors;
}

///A trait for performing division with with remainder
pub trait EuclideanDiv: Sized {
    type Naturals: Natural;

    fn euclid_norm(&self) -> Self::Naturals;
    fn div_euc(self, rhs: Self) -> Self;
    fn rem_euc(self, rhs: Self) -> Self;
    fn div_alg(self, rhs: Self) -> (Self, Self);
}
pub trait Exponential: Sized {
    ///
    ///The exponential of this ring element
    ///
    ///Here, `exp(x)` is defined such that:
    /// * `exp(x+y) = exp(x)*exp(y)` for all `x` and `y` where `x*y=y*x`
    /// * `exp(x) != 0`
    /// * `exp(x)` is continuous (if applicable)
    /// * `d/dx exp(x)|ₓ₌₁ = 1` (if applicable)
    ///
    ///For most structures, this function is equivalent to the infinite series Σ x<sup>n</sup>/n!
    ///
    fn exp(self) -> Self;

    ///
    ///An inverse of [exp(x)](Exponential::exp) where `ln(1) = 0`
    ///
    ///This returns a `None` value whenever the inverse does not exist for the given input.
    ///
    /// ## Uniqueness and Continuity
    ///
    ///Do note, however, that for almost all non-[Real](crate::analysis::Real) structures, this function
    ///is not unique and can **never** be continuous. Of course, some of this ambiguity is resolved by
    ///stipulating that `ln(1) = 0`, but even so, some remains,
    ///and so, it is entirely up to the implementor to any specific canonical form if applicable.
    ///
    ///For example, the [Complex](crate::analysis::Complex) numbers, the natural logarithm *must* be discontinuous somewhere,
    ///and there are infinitely many choices as to where that is. However, usually, this ambiguity
    ///is removed by taking the imaginary component of the result between -π and π and setting
    ///the discontinuity to be on the negative real axis
    ///
    fn try_ln(self) -> Option<Self>;
}

///A commutative and additive monoid with a distributive and associative multiplication operation
pub trait Semiring = Distributive + AddMonoid + AddCommutative + MulSemigroup;
///A semiring with an identity element
pub trait UnitalSemiring = Semiring + MulMonoid;
///A unital semiring where multiplication is commutative
pub trait CommutativeSemiring = UnitalSemiring + MulCommutative;
///A semiring with a multiplicative inverse
pub trait DivisionSemiring = UnitalSemiring + MulGroup;

///An additive abelian group with a distributive and associative multiplication operation
pub trait Ring = Distributive + AddAbelianGroup + MulSemigroup;
///A ring with an identity element
pub trait UnitalRing = Ring + MulMonoid;
///A unital ring where multiplication is commutative
pub trait CommutativeRing = UnitalRing + MulCommutative;
///A ring with a multiplicative inverse
pub trait DivisionRing = UnitalRing + MulGroup;
///A ring with an exponential operation
pub trait ExponentialRing = UnitalRing + Exponential;

///A unital semiring with no pairs of nonzero elements that multiply to zero
pub trait Semidomain = UnitalSemiring + Divisibility + NoZeroDivisors;
///A semidomain that is commutative
pub trait IntegralSemidomain = Semidomain + MulCommutative;
///An integral semidomain where every pair of elements has a greatest common divisor
pub trait GCDSemidomain = IntegralSemidomain + GCD;
///A GCD semidomain where every pair of elements is uniquely factorizable into irreducible elements (up to units)
pub trait UFSemidomain = GCDSemidomain + UniquelyFactorizable;
///A UF semidomain with a division algorithm for dividing with a remainder
pub trait EuclideanSemidomain = UFSemidomain + EuclideanDiv;

///A unital ring with no pairs of nonzero elements that multiply to zero
pub trait Domain = UnitalRing + Divisibility + NoZeroDivisors;
///A domain that is commutative
pub trait IntegralDomain = Domain + MulCommutative;
///A commutative ring where every pair of elements has a greatest common divisor
pub trait GCDDomain = IntegralDomain + GCD;
///A commutative ring where every pair of elements has a weighted sum to their GCD
pub trait BezoutDomain = GCDDomain + Bezout;
///A commutative ring that is uniquely factorizable into irreducible (up to units)
pub trait UFD = GCDDomain + UniquelyFactorizable;
///
///An integral domain where every ideal is generated by one element
///
///ie. a UFD that is Bezout
///
pub trait PID = UFD + BezoutDomain;
///A commutative ring with a division algorithm for dividing with a remainder
pub trait EuclideanDomain = PID + EuclideanDiv;

///A set that is both an additive and multiplicative abelian group where multiplication distributes
pub trait Field = CommutativeRing + MulGroup;
///A field with an exponential operation
pub trait ExponentialField = Field + Exponential;

//
//Implementation for primitives
//

macro_rules! impl_dist {
    ($($t:ty)*) => {
        $(
            impl Distributive for $t{}
            impl Distributive for ::core::num::Wrapping<$t>{}
        )*
    };
}
impl_dist!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64);

macro_rules! impl_for_field {
    ($($t:ty)*) => {
        $(
            impl Divisibility for $t {
                #[inline(always)] fn divides(self, _rhs: Self) -> bool {true}
                #[inline(always)] fn divide(self, rhs: Self) -> Option<Self> {Some(rhs / self)}
                #[inline(always)] fn unit(&self) -> bool {true}
                #[inline(always)] fn inverse(self) -> Option<Self> {Some(self.inv())}
            }

            impl NoZeroDivisors for $t {}
            impl UniquelyFactorizable for $t {}

        )*
    }
}

impl_for_field!(f32 f64);

///Uses the [Euclidean Algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm)
///to find the [GCD] of two ring elements using [division with remainder](EuclideanDiv)
pub fn euclidean<T>(lhs: T, rhs: T) -> T
where
    T: CommutativeSemiring + EuclideanDiv,
{
    fn euclid<U>(a: U, b: U) -> U
    where
        U: CommutativeSemiring + EuclideanDiv,
    {
        let r = a.rem_euc(b.clone());
        if r.is_zero() {
            b
        } else {
            euclid(b, r)
        }
    }

    if lhs.is_zero() || rhs.is_zero() {
        return T::zero();
    }

    if lhs.euclid_norm() >= rhs.euclid_norm() {
        euclid(lhs, rhs)
    } else {
        euclid(rhs, lhs)
    }
}

///
///Uses the [Extended Euclidean Algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
///to find the [GCD] of two ring elements _and_ their [bezout coefficients](Bezout) using
///[division with remainder](EuclideanDiv)
///
pub fn extended_euclidean<T>(lhs: T, rhs: T) -> (T, T, T)
where
    T: CommutativeRing + EuclideanDiv,
{
    fn euclid<U>(a: U, b: U) -> (U, U, U)
    where
        U: CommutativeRing + EuclideanDiv,
    {
        let (q, r) = a.div_alg(b.clone());
        if r.is_zero() {
            (U::zero(), U::one(), b)
        } else {
            let (x1, y1, g) = euclid(b, r);
            (y1.clone(), x1 - q * y1, g)
        }
    }

    if lhs.is_zero() || rhs.is_zero() {
        return (T::zero(), T::zero(), T::zero());
    }

    if lhs.euclid_norm() >= rhs.euclid_norm() {
        euclid(lhs, rhs)
    } else {
        let (y, x, g) = euclid(rhs, lhs);
        (x, y, g)
    }
}

///
///Determines if a given Natural number is [prime](Primality) using the
///[Miller-Rabin primality test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
///
///The algorithm works in essence by searching for counter-examples to Fermat's Little theorem, ie,
///that `a^(p-1) = 1` for all `a` in `Z/pZ` when `p` is prime. In general, this tactic only gives a
///way to prove a number is _not_ prime, but with a few modifications to the check and by picking
///the right examples, it is possible to turn this into a deterministic test. \*
///
///Furthermore, this particular algorithm has the surprising benefit of having a runtime that is
///polynomial in the number of bits in the input. Of course, this does not guarantee that this function
///is particularly "fast" per se, but in testing, the algorithm runs reasonable fast for all primitive
///integer types.
///
///\* It is important to note that the particular method used to achieve a Deterministic Miller Robin
///algorithm in polynomial time, _technically_ depends on the Generalized Riemann Hypothesis. So I
///guess that means that for super huge numbers, this technically _could_ give a false positive... ¯\\\_(ツ)\_/¯
///But hey, what _else_ is there? The AKS Primality Test?
///
pub fn miller_rabin<Z: Natural>(n: Z) -> bool {
    //trivial cases
    if n <= Z::one() {
        return false;
    }
    if n == Z::two() {
        return true;
    }
    if n.even() {
        return false;
    }

    //decompose n-1 (ie. -1 in Z/nZ) into a product of the form d*2^s
    let mut d = n.clone() - Z::one();
    let mut s = Z::zero();
    while d.even() {
        s = s + Z::one();
        d = d.div_two();
    }

    #[inline]
    fn witness<N: Natural>(witness: u128, d: N, s: N, n: N) -> bool {
        _witness(N::try_from(witness).unwrap_or(N::zero()), d, s, n)
    }

    fn _witness<N: Natural>(mut a: N, mut d: N, mut s: N, n: N) -> bool {
        let mut r = a.clone();
        d = d - N::one();
        while d > N::zero() {
            if d.even() {
                d = d.div_two();
                a = (a.clone() * a) % n.clone();
            } else {
                d = d - N::one();
                r = (r * a.clone()) % n.clone();
            }
        }

        if r.is_one() {
            true
        } else {
            loop {
                if r == n.clone() - N::one() {
                    return true;
                }

                if s.is_zero() {
                    break;
                } else {
                    s = s - N::one();
                    r = (r.clone() * r) % n.clone();
                }
            }
            false
        }
    }

    //the breakpoints for each set of sufficient witnesses
    let b1 = Z::from_u32(2047u32);
    let b2 = Z::from_u32(1373653u32);
    let b3 = Z::from_u32(9080191u32);
    let b4 = Z::from_u32(25326001u32);
    let b5 = Z::from_u64(3215031751u64);
    let b6 = Z::from_u64(4759123141u64);
    let b7 = Z::from_u64(1122004669633u64);
    let b8 = Z::from_u64(2152302898747u64);
    let b9 = Z::from_u64(3474749660383u64);
    let b10 = Z::from_u64(341550071728321u64);
    let b11 = Z::from_u64(3825123056546413051u64);
    let b12 = Z::from_u128(318665857834031151167461u128);
    let b13 = Z::from_u128(3317044064679887385961981u128);

    if b1.map_or(true, |x| n < x) {
        witness(2, d.clone(), s.clone(), n.clone())
    } else if b2.map_or(true, |x| n < x) {
        witness(2, d.clone(), s.clone(), n.clone()) && witness(3, d.clone(), s.clone(), n.clone())
    } else if b3.map_or(true, |x| n < x) {
        witness(31, d.clone(), s.clone(), n.clone()) && witness(73, d.clone(), s.clone(), n.clone())
    } else if b4.map_or(true, |x| n < x) {
        witness(2, d.clone(), s.clone(), n.clone())
            && witness(3, d.clone(), s.clone(), n.clone())
            && witness(5, d.clone(), s.clone(), n.clone())
    } else if b5.map_or(true, |x| n < x) {
        witness(2, d.clone(), s.clone(), n.clone())
            && witness(3, d.clone(), s.clone(), n.clone())
            && witness(5, d.clone(), s.clone(), n.clone())
            && witness(7, d.clone(), s.clone(), n.clone())
    } else if b6.map_or(true, |x| n < x) {
        witness(2, d.clone(), s.clone(), n.clone())
            && witness(7, d.clone(), s.clone(), n.clone())
            && witness(61, d.clone(), s.clone(), n.clone())
    } else if b7.map_or(true, |x| n < x) {
        witness(2, d.clone(), s.clone(), n.clone())
            && witness(13, d.clone(), s.clone(), n.clone())
            && witness(23, d.clone(), s.clone(), n.clone())
            && witness(1662803, d.clone(), s.clone(), n.clone())
    } else if b13.map_or(true, |x| n < x) {
        witness(2, d.clone(), s.clone(), n.clone())
            && witness(3, d.clone(), s.clone(), n.clone())
            && witness(5, d.clone(), s.clone(), n.clone())
            && witness(7, d.clone(), s.clone(), n.clone())
            && witness(11, d.clone(), s.clone(), n.clone())
            && if b8.map_or(false, |x| n >= x) {
                witness(13, d.clone(), s.clone(), n.clone())
            } else {
                true
            }
            && if b9.map_or(false, |x| n >= x) {
                witness(17, d.clone(), s.clone(), n.clone())
            } else {
                true
            }
            && if b10.map_or(false, |x| n >= x) {
                witness(19, d.clone(), s.clone(), n.clone())
                    && witness(23, d.clone(), s.clone(), n.clone())
            } else {
                true
            }
            && if b11.map_or(false, |x| n >= x) {
                witness(29, d.clone(), s.clone(), n.clone())
                    && witness(31, d.clone(), s.clone(), n.clone())
                    && witness(37, d.clone(), s.clone(), n.clone())
            } else {
                true
            }
            && if b12.map_or(false, |x| n >= x) {
                witness(41, d.clone(), s.clone(), n.clone())
            } else {
                true
            }
    } else {
        //in general, we need to check every witness below 2*ln(n)^2
        let mut a = Z::two();
        while Z::try_from(3.pow_n(a.clone().div_two())).unwrap_or(n.clone()) < n.clone().mul_two() {
            if !_witness(a.clone(), d.clone(), s.clone(), n.clone()) {
                return false;
            }
            a = a + Z::one();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::*;

    //TODO: add tests for euclidean and extended_euclidean

    #[test]
    fn primality() {
        assert!(18446744073709551557u64.prime());
        assert!(!18446744073709551559u64.prime());
        assert!(!18446744073709551555u64.prime());

        assert!(2147483647u32.prime());
        assert!(!2147483649u32.prime());
        assert!(!2147483645u32.prime());

        assert!(65521u16.prime());
        assert!(65519u16.prime());
        assert!(!65523u16.prime());

        assert!(251u8.prime());
        assert!(!253u8.prime());
        assert!(!249u8.prime());

        assert!(13u8.prime());
        assert!(!15u8.prime());
    }

    #[test]
    fn factor() {
        #[cfg(feature = "std")]
        {
            assert_eq!(91u8.factors().collect::<Vec<_>>(), vec![7, 13]);
            assert_eq!((-91i8).factors().collect::<Vec<_>>(), vec![-1, 7, 13]);

            assert_eq!(360u16.factors().collect::<Vec<_>>(), vec![2, 2, 2, 3, 3, 5]);
            assert_eq!(
                (-360i16).factors().collect::<Vec<_>>(),
                vec![-1, 2, 2, 2, 3, 3, 5]
            );

            assert_eq!(
                1971813u32.factors().collect::<Vec<_>>(),
                vec![3, 17, 23, 41, 41]
            );
            assert_eq!(
                (-1971813i32).factors().collect::<Vec<_>>(),
                vec![-1, 3, 17, 23, 41, 41]
            );

            assert_eq!(
                1971813u32.factors().collect::<Vec<_>>(),
                vec![3, 17, 23, 41, 41]
            );
            assert_eq!(
                (-1971813i32).factors().collect::<Vec<_>>(),
                vec![-1, 3, 17, 23, 41, 41]
            );

            assert_eq!(
                0x344CAF57AB24A9u64.factors().collect::<Vec<_>>(),
                vec![101, 101, 103, 103, 107, 107, 109, 109]
            );
            assert_eq!(
                (-0x344CAF57AB24A9i64).factors().collect::<Vec<_>>(),
                vec![-1, 101, 101, 103, 103, 107, 107, 109, 109]
            );
        }

        fn factors_slice<Z: IntegerSubset + Factorizable>(x: Z, dest: &mut [Z]) -> usize {
            let mut i = 0;
            for f in x.factors() {
                if i < dest.len() {
                    dest[i] = f;
                    i += 1;
                } else {
                    return i;
                }
            }
            return i;
        }

        let mut factors = [0xFF; 3];

        //test 0 returns 0
        assert_eq!(factors_slice(0u8, &mut factors), 1);
        assert_eq!(&factors, &[0, 0xFF, 0xFF]);

        //test 1 returns 1
        assert_eq!(factors_slice(1u8, &mut factors), 0);
        assert_eq!(&factors, &[0, 0xFF, 0xFF]);

        //test the algorithm stopping at the end of the array
        assert_eq!(factors_slice(210u8, &mut factors), 3);
        assert_eq!(&factors, &[2, 3, 5]); //skips 7

        let mut factors = [0; 10];

        //test -1 giving -1
        assert_eq!(factors_slice(-1i64, &mut factors), 1);
        assert_eq!(&factors, &[-1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(factors_slice(-0x344CAF57AB24A9i64, &mut factors), 9);
        assert_eq!(&factors, &[-1, 101, 101, 103, 103, 107, 107, 109, 109, 0]);

        assert_eq!(factors_slice(0x344CAF57AB24A9i64, &mut factors), 8);
        assert_eq!(&factors, &[101, 101, 103, 103, 107, 107, 109, 109, 109, 0]);
    }
}
