use heightmap::Heightmap;
use image::{ImageBuffer, Luma};
use noise2d::Noise2d;

pub trait ToImage {
    fn to_image(&self) -> ImageBuffer<Luma<u8>, Vec<u8>>;
}


impl ToImage for Noise2d {
    fn to_image(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut imbuf = ImageBuffer::new(256, 256);

        for y in 0..256 {
            for x in 0..256 {
                let u = (x as f64) / 64.0;
                let v = (y as f64) / 64.0;

                let v = (self.at(u, v) * 255.0) as u8;
                imbuf.put_pixel(x as u32, y as u32, Luma([v]));
            }
        }

        imbuf
    }
}

impl ToImage for Heightmap {
    fn to_image(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        ImageBuffer::from_fn(self.width(),
                             self.height(),
                             |x, y| Luma([(self.get(x, y) * 255.0) as u8]))
    }
}
