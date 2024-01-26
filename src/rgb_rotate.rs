use std::simd::f32x4;
use std::simd::prelude::SimdFloat;

mod asm {
    use std::arch::global_asm;

    global_asm!(include_str!("rgb_rotate.s"));

    extern "C" {
        pub fn rotate_pixels(source: *const u8, target: *mut u8, matrix: *const f32, length: usize);
    }
}

pub struct RGBRotate {
    matrix: [f32; 9],
    simd_matrix: [f32x4; 3],
}

macro_rules! clamp {
    ($value:expr) => {{
        if $value < 0.0 {
            0
        } else if $value > 255.0 {
            255
        } else {
            $value as u8
        }
    }};
}

impl RGBRotate {
    pub fn new(angle: i32) -> RGBRotate {
        let angle = (angle as f32).to_radians();

        let cosv = angle.cos();
        let sinv = angle.sin();

        let matrix = [
            // Reds
            0.213 + cosv * 0.787 - sinv * 0.213,
            0.715 - cosv * 0.715 - sinv * 0.715,
            0.072 - cosv * 0.072 + sinv * 0.928,
            // Greens
            0.213 - cosv * 0.213 + sinv * 0.143,
            0.715 + cosv * 0.285 + sinv * 0.140,
            0.072 - cosv * 0.072 - sinv * 0.283,
            // Blues
            0.213 - cosv * 0.213 - sinv * 0.787,
            0.715 - cosv * 0.715 + sinv * 0.715,
            0.072 + cosv * 0.928 + sinv * 0.072,
        ];

        let simd_matrix = [
            f32x4::from([matrix[0], matrix[1], matrix[2], 0.0]),
            f32x4::from([matrix[3], matrix[4], matrix[5], 0.0]),
            f32x4::from([matrix[6], matrix[7], matrix[8], 0.0]),
        ];

        RGBRotate {
            matrix,
            simd_matrix,
        }
    }

    pub fn rotate_pixels(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(bytes.len());

        for pixel in bytes.chunks_exact(3) {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            let rx = r * self.matrix[0] + g * self.matrix[1] + b * self.matrix[2];
            let gx = r * self.matrix[3] + g * self.matrix[4] + b * self.matrix[5];
            let bx = r * self.matrix[6] + g * self.matrix[7] + b * self.matrix[8];

            result.push(clamp!(rx));
            result.push(clamp!(gx));
            result.push(clamp!(bx));
        }

        result
    }

    pub fn rotate_pixels_portable_simd(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(bytes.len());

        for pixel in bytes.chunks_exact(3) {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            let pixel = f32x4::from([r, g, b, 0.0]);

            let rx = (pixel * self.simd_matrix[0]).reduce_sum();
            let gx = (pixel * self.simd_matrix[1]).reduce_sum();
            let bx = (pixel * self.simd_matrix[2]).reduce_sum();

            result.push(clamp!(rx));
            result.push(clamp!(gx));
            result.push(clamp!(bx));
        }

        result
    }

    pub fn rotate_pixels_asm(&self, bytes: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = vec![0; bytes.len()];

        unsafe {
            asm::rotate_pixels(
                bytes.as_ptr(),
                result.as_mut_ptr(),
                self.matrix.as_ptr(),
                bytes.len(),
            );
        }

        result
    }
}
