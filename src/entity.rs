pub trait Bounds {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn w(&self) -> f64;
    fn h(&self) -> f64;
    fn bounds(&self) -> [f64; 4] {
        [self.x(), self.y(), self.w(), self.h()]
    }
}

pub trait Speed {
    fn dx(&self) -> f64;
    fn dy(&self) -> f64;
}
