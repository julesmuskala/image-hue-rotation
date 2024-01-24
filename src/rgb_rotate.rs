pub struct RGBRotate {
    matrix: [f64; 9],
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
        let angle = (angle as f64).to_radians();

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

        RGBRotate { matrix }
    }

    pub fn rotate_pixels(&self, bytes: &[u8]) -> Vec<u8> {
        bytes
            .chunks_exact(3)
            .map(|pixel| {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                let rx = r as f64 * self.matrix[0]
                    + g as f64 * self.matrix[1]
                    + b as f64 * self.matrix[2];

                let gx = r as f64 * self.matrix[3]
                    + g as f64 * self.matrix[4]
                    + b as f64 * self.matrix[5];

                let bx = r as f64 * self.matrix[6]
                    + g as f64 * self.matrix[7]
                    + b as f64 * self.matrix[8];

                [clamp!(rx), clamp!(gx), clamp!(bx)]
            })
            .flatten()
            .collect()
    }
}
