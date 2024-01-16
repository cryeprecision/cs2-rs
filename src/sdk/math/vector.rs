#[repr(C)]
#[derive(Debug)]
pub struct Vec2 {
    pub data: [f32; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct Vec3 {
    pub data: [f32; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct Vec4 {
    pub data: [f32; 4],
}
