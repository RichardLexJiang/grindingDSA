pub fn quick_pow<T>(base: T, exponent: u128, modd: T) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + One + Copy,
{
    let mut result = T::one();
    if exponent == 0 {
        return result;
    };
    let mut base = base % modd;
    let mut x = exponent;
    while x != 0 {
        if x & 1 != 0 {
            result = (result * base) % modd;
        }
        base = (base * base) % modd;
        x = x >> 1;
    }
    return result;
}

/// I get this part code from crate:num_traits
pub trait One: Sized + std::ops::Mul<Self, Output = Self> {
    fn one() -> Self;
}

macro_rules! one_impl {
    ($t:ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }
        }
    };
}

one_impl!(usize, 1);
one_impl!(u8, 1);
one_impl!(u16, 1);
one_impl!(u32, 1);
one_impl!(u64, 1);
one_impl!(u128, 1);
one_impl!(isize, 1);
one_impl!(i8, 1);
one_impl!(i16, 1);
one_impl!(i32, 1);
one_impl!(i64, 1);
one_impl!(i128, 1);
one_impl!(f32, 1.0);
one_impl!(f64, 1.0);

fn normal_pow(a: i128, b: i128, modd: i128) -> i128 {
    let mut result = 1;
    for _ in 0..b {
        result = (result * a) % modd;
    }
    result
}

#[test]
fn test() {
    assert_eq!(quick_pow(11111111111111u64, 0, 100000), 1);
    assert_eq!(quick_pow(2, 3, 100000), 8);
    assert_eq!(
        quick_pow(641725, 225225174, 1e9 as i128 + 7),
        normal_pow(641725, 225225174, 1e9 as i128 + 7)
    );
    assert_eq!(
        quick_pow(641725, 225225174 + 1, 1e9 as i128 + 7),
        normal_pow(641725, 225225174 + 1, 1e9 as i128 + 7)
    );
}
