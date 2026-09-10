#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Radians(f32);

impl Radians {
    pub fn from_radians(radians: f32) -> Self {
        Self(radians)
    }
    pub fn from_degrees(degrees: f32) -> Self {
        Self(degrees.to_radians())
    }
    pub fn as_degrees(self) -> f32 {
        self.0.to_degrees()
    }

    pub const fn as_radians(self) -> f32 {
        self.0
    }
}
