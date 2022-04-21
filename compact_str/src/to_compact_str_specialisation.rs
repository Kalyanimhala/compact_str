use super::CompactStr;

use castaway::cast;

pub(super) fn to_compact_str_specialised<T>(val: &T) -> Option<CompactStr> {
    #[cfg(feature = "to-compact-str-int-spec")]
    if let Some(compact_str) = int_spec::to_compact_str_specialised(val) {
        return Some(compact_str);
    }

    #[cfg(feature = "to-compact-str-float-spec")]
    if let Some(compact_str) = float_spec::to_compact_str_specialised(val) {
        return Some(compact_str);
    }

    if let Ok(boolean) = cast!(val, &bool) {
        Some(CompactStr::new(if *boolean { "true" } else { "false" }))
    } else if let Ok(string) = cast!(val, &String) {
        Some(CompactStr::new(&*string))
    } else if let Ok(character) = cast!(val, &char) {
        Some(CompactStr::new(character.encode_utf8(&mut [0; 4][..])))
    } else {
        None
    }
}

#[cfg(feature = "to-compact-str-int-spec")]
mod int_spec {
    use super::*;

    use core::num;
    use itoa::{Buffer, Integer};

    fn int_to_compact_str(int: impl Integer) -> CompactStr {
        CompactStr::new(Buffer::new().format(int))
    }

    macro_rules! specialise {
        ($val: expr, $int: ty, $nonzero_int: ty) => {
            if let Ok(int) = cast!($val, &$int) {
                return Some(int_to_compact_str(*int));
            } else if let Ok(nonzero_int) = cast!($val, &$nonzero_int) {
                return Some(int_to_compact_str(nonzero_int.get()));
            }
        };
    }

    pub(super) fn to_compact_str_specialised<T>(val: &T) -> Option<CompactStr> {
        specialise!(val, i8, num::NonZeroI8);
        specialise!(val, u8, num::NonZeroU8);

        specialise!(val, i16, num::NonZeroI16);
        specialise!(val, u16, num::NonZeroU16);

        specialise!(val, i32, num::NonZeroI32);
        specialise!(val, u32, num::NonZeroU32);

        specialise!(val, i64, num::NonZeroI64);
        specialise!(val, u64, num::NonZeroU64);

        specialise!(val, i128, num::NonZeroI128);
        specialise!(val, u128, num::NonZeroU128);

        specialise!(val, isize, num::NonZeroIsize);

        // castaway::LifetimeFree didn't implement for `num::NonZeroUsize`
        // and will be fixed in https://github.com/sagebind/castaway/pull/7.
        specialise!(val, usize, num::NonZeroIsize);

        None
    }
}

#[cfg(feature = "to-compact-str-float-spec")]
mod float_spec {
    use super::*;

    use ryu::{Buffer, Float};

    #[inline(always)]
    fn float_to_compact_str(float: impl Float) -> CompactStr {
        CompactStr::new(Buffer::new().format(float))
    }

    #[inline(always)]
    pub(super) fn to_compact_str_specialised<T>(val: &T) -> Option<CompactStr> {
        if let Ok(float) = cast!(val, &f32) {
            Some(float_to_compact_str(*float))
        } else if let Ok(float) = cast!(val, &f64) {
            Some(float_to_compact_str(*float))
        } else {
            None
        }
    }
}
