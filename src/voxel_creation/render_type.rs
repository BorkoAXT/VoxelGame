pub enum RenderType {
    None,
    Opaque,
    Cutout,
    Translucent,
    Liquid,
}
impl Default for RenderType {
    fn default() -> Self {
        Self::Opaque
    }
}
