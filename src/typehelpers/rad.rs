#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rad {
    radians: f32,
}

impl Rad {
    pub const fn new(radians: f32) -> Self {
        Self { radians }
    }

    pub fn from_degrees(degrees: f32) -> Self {
        Self {
            radians: degrees.to_radians(),
        }
    }

    pub const fn as_radians(self) -> f32 {
        self.radians
    }

    pub fn as_degrees(self) -> f32 {
        self.radians.to_degrees()
    }
}
