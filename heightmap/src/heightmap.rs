pub struct Heightmap {
    width: u32,
    height: u32,
    data: Box<[f64]>,
}

impl Heightmap {
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn get(&self, x: u32, y: u32) -> f64 {
        self.data[(y * self.width + x) as usize]
    }

    fn minmax(&self) -> Option<(f64, f64)> {
        let mut it = self.data.iter();

        it.next().map(|sel| {
            let mut min = sel;
            let mut max = sel;

            for x in it {
                if x < min {
                    min = x
                };
                if x > max {
                    max = x
                };
            }

            (*min, *max)
        })
    }

    pub fn normalize(&mut self) {
        if let Some((min, max)) = self.minmax() {
            for x in self.data.iter_mut() {
                *x = (*x - min) / (max - min);
            }
        }
    }

    pub fn flatten(&mut self) {
        let h = |x: f64| (x - 0.5) * (x - 0.5) * (if x < 0.5 { -2.0 } else { 2.0 }) + 0.5;

        for x in self.data.iter_mut() {
            *x = h(*x);
        }
    }

    pub fn submap(&self, x: u32, y: u32, width: u32, height: u32) -> Heightmap {
        if x + width > self.width {
            panic!("Width out of bounds: {} + {} > {}", x, width, self.width)
        }
        if y + height > self.height {
            panic!("Height out of bounds: {} + {} > {}", y, height, self.height)
        }

        if width == self.width && height == self.height {
            Heightmap {
                width,
                height,
                data: self.data.clone(),
            }
        } else {
            Heightmap {
                width,
                height,
                data: self.data
                    .chunks(self.width() as usize)
                    .skip(y as usize)
                    .take((height - y) as usize)
                    .flat_map(|row| {
                                  row.iter()
                                      .skip(x as usize)
                                      .take((width - x) as usize)
                                      .cloned()
                              })
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            }
        }
    }

    pub fn heights(&self) -> impl Iterator<Item = &f64> {
        self.data.iter()
    }
}

pub fn heightmap_from_vec(width: u32, height: u32, data: Vec<f64>) -> Heightmap {
    Heightmap {
        width,
        height,
        data: data.into_boxed_slice(),
    }
}

pub fn heightmap_from_iter<I>(width: u32, height: u32, iter: I) -> Heightmap
    where I: Iterator<Item = f64>
{
    let vec: Vec<f64> = iter.collect();
    heightmap_from_vec(width, height, vec)
}
