#[repr(C)]
#[derive(Debug)]
pub struct Matrix33 {
    pub data: [[f32; 3]; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct Matrix34 {
    pub data: [[f32; 3]; 4],
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct Matrix34a {
    pub data: [[f32; 3]; 4],
}
