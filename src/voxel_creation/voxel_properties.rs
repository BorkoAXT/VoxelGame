use super::render_type::RenderType;
pub struct VoxelProperties {
    pub is_collidable: bool,
    pub is_solid: bool,
    pub is_opaque: bool,
    pub is_replaceable: bool,
    pub is_emissive: bool,
    pub render_type: RenderType,
}
impl Default for VoxelProperties {
    fn default() -> Self {
        return Self {
            is_collidable: true,
            is_solid: true,
            is_opaque: true,
            is_replaceable: true,
            is_emissive: true,
            render_type: Default::default(),
        };
    }
}
