pub trait Midpoint {
    fn midpoint(self, rhs: Self) -> Self;
}

macro_rules! impl_midpoint_for {
    (original_type: $original_type:ty, unsigned_type: $unsigned_type:ty) => {
        impl Midpoint for $original_type {
            fn midpoint(self, rhs: Self) -> Self {
                let lhs = self;
                let lhs_unsigned = lhs as $unsigned_type;
                let rhs_unsigned = rhs as $unsigned_type;

                if lhs > rhs {
                    lhs - (lhs_unsigned.wrapping_sub(rhs_unsigned) / 2) as $original_type
                } else {
                    lhs + (rhs_unsigned.wrapping_sub(lhs_unsigned) / 2) as $original_type
                }
            }
        }
    };
}

impl_midpoint_for!(original_type: i8, unsigned_type: u8);
impl_midpoint_for!(original_type: u8, unsigned_type: u8);

impl_midpoint_for!(original_type: i16, unsigned_type: u16);
impl_midpoint_for!(original_type: u16, unsigned_type: u16);

impl_midpoint_for!(original_type: i32, unsigned_type: u32);
impl_midpoint_for!(original_type: u32, unsigned_type: u32);

impl_midpoint_for!(original_type: i64, unsigned_type: u64);
impl_midpoint_for!(original_type: u64, unsigned_type: u64);

impl_midpoint_for!(original_type: i128, unsigned_type: u128);
impl_midpoint_for!(original_type: u128, unsigned_type: u128);
