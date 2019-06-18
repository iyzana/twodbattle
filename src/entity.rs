use graphics::Context;

pub trait Bounds {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn w(&self) -> f64;
    fn h(&self) -> f64;
    fn bounds(&self) -> [f64; 4] {
        [self.x(), self.y(), self.w(), self.h()]
    }
    fn scale(&self, c: &Context) -> [f64; 4] {
        let [w, h] = c.get_view_size();
        [
            self.x() / 1920.0 * w,
            self.y() / 1080.0 * h,
            self.w() / 1080.0 * w,
            self.h() / 1080.0 * h,
        ]
    }
}

impl Bounds for [f64; 4] {
    fn x(&self) -> f64 {
        self[0]
    }
    fn y(&self) -> f64 {
        self[1]
    }
    fn w(&self) -> f64 {
        self[2]
    }
    fn h(&self) -> f64 {
        self[3]
    }
}

pub trait Speed {
    fn dx(&self) -> f64;
    fn dy(&self) -> f64;
}
