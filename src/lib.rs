#![no_std]

mod i8 {
    // Python: [math.floor((math.cos(a/256.0*math.tau))/2*255) for a in range(256)]
    pub(super) const COS_TABLE_256: [i8; 256] =
        [127, 127, 127, 127, 126, 126, 126, 125, 125, 124, 123, 122, 122, 121, 120, 118, 117, 116, 115, 113, 112, 110, 109, 107, 106, 104, 102, 100, 98, 96, 94, 92, 90, 87, 85, 83, 80, 78, 75, 73, 70, 68, 65, 62, 60, 57, 54, 51, 48, 45, 42, 39, 37, 34, 30, 27, 24, 21, 18, 15, 12, 9, 6, 3, 0, -4, -7, -10, -13, -16, -19, -22, -25, -28, -31, -35, -38, -40, -43, -46, -49, -52, -55, -58, -61, -63, -66, -69, -71, -74, -76, -79, -81, -84, -86, -88, -91, -93, -95, -97, -99, -101, -103, -105, -107, -108, -110, -111, -113, -114, -116, -117, -118, -119, -121, -122, -123, -123, -124, -125, -126, -126, -127, -127, -127, -128, -128, -128, -128, -128, -128, -128, -127, -127, -127, -126, -126, -125, -124, -123, -123, -122, -121, -119, -118, -117, -116, -114, -113, -111, -110, -108, -107, -105, -103, -101, -99, -97, -95, -93, -91, -88, -86, -84, -81, -79, -76, -74, -71, -69, -66, -63, -61, -58, -55, -52, -49, -46, -43, -40, -38, -35, -31, -28, -25, -22, -19, -16, -13, -10, -7, -4, -1, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 34, 37, 39, 42, 45, 48, 51, 54, 57, 60, 62, 65, 68, 70, 73, 75, 78, 80, 83, 85, 87, 90, 92, 94, 96, 98, 100, 102, 104, 106, 107, 109, 110, 112, 113, 115, 116, 117, 118, 120, 121, 122, 122, 123, 124, 125, 125, 126, 126, 126, 127, 127, 127];
}

pub trait Trigonometry {
    type Output;

    fn sin(self) -> Self::Output;
    fn cos(self) -> Self::Output;
}

impl Trigonometry for u8 {
    type Output = i8;

    fn sin(self) -> i8 {
        i8::COS_TABLE_256[((self as u16 + 64u16) % 256) as usize]
    }

    fn cos(self) -> i8 {
        i8::COS_TABLE_256[self as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cos() {
        assert_eq!(0u8.cos(), 127);
        assert_eq!(63u8.cos(), 3);
        assert_eq!(127u8.cos(), -128);
        assert_eq!(191u8.cos(), -4);
    }

    #[test]
    fn sin() {
        assert_eq!(0u8.sin(), 0);
        assert_eq!(63u8.sin(), -128);
        assert_eq!(127u8.sin(), -4);
        assert_eq!(191u8.sin(), 127);
    }

    #[test]
    fn circle() {
        for i in 0..255 {
            let cos = i.cos() as i32;
            let sin = i.sin() as i32;
            let r = ((cos * cos + sin * sin) as f64).sqrt();
            assert!(r >= 126f64);
            assert!(r <= 129f64);
        }
    }
}
