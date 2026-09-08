pub const VOXEL_SIZE: f32 = 0.5;
pub const HALF_VOXEL: f32 = VOXEL_SIZE / 2.0;
pub const CHUNK_SIZE: usize = 16;
pub const INDICES_ARRAY: [u16; 36] = [
    0, 1, 2, 2, 3, 0, // face1
    4, 5, 6, 6, 7, 4, // face2
    0, 4, 7, 7, 3, 0, // face3
    1, 5, 6, 6, 2, 1, // face4
    3, 2, 6, 6, 7, 3, // face5
    0, 1, 5, 5, 4, 0, // face 6
];
