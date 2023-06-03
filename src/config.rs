

// translation
pub const X_SMALL: i32 = 5;
pub const Y_SMALL: i32 = 0;
pub const Z_SMALL: i32 = -4;
pub const RESOLUTION_SMALL: (f32,f32) = (600.,600.);
pub fn x_t(x: i32) -> f32 { (x + X_SMALL) as f32 } 
pub fn y_t(x: i32) -> f32 { (x + Y_SMALL) as f32 }
pub fn z_t(x: i32) -> f32 { (x + Z_SMALL) as f32 }
pub const LIGHT_SMALL: (f32,f32,f32) =  (4.0, 8.0, 0.0);

pub const X_LARGE: i32 = 0;
pub const Y_LARGE: i32 = 0;
pub const Z_LARGE: i32 = 0;
pub const RESOLUTION_LARGE: (f32,f32) = (1600.,1600.);
// pub fn x_t(x: i32) -> f32 { (x + X_LARGE) as f32 } 
// pub fn y_t(x: i32) -> f32 { (x + Y_LARGE) as f32 }
// pub fn z_t(x: i32) -> f32 { (x + Z_LARGE) as f32 }
pub const LIGHT_LARGE: (f32,f32,f32) =  (4.0, 8.0, 4.0);
