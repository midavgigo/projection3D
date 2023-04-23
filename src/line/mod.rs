pub mod line{
    use crate::vector::vector::V3;

    pub struct Line{
        pos:V3,
        dir:V3
    }

    impl Line {
        pub fn mk_2dots(beg:&V3, end:&V3) -> Self{
            let (x1, y1, z1) = beg.get();
            let (x2, y2, z2) = end.get();
            let (kx, ky, kz) = (x2-x1, y2-y1, z2-z1);
            let dir = V3::make((kx, ky, kz));
            Self{
                dir,
                pos:beg.clone()
            }
        }
        pub fn get_pos(&self)->V3{
            self.pos.clone()
        }
        pub fn get_vec(&self)->V3{
            self.dir.clone()
        }
        pub fn distance(&self, dot:&V3)->f32{
            let vec = dot.clone().add(&self.pos.neg());
            let (x, y, z) = vec.get();
            let (kx, ky, kz) = self.dir.get();
            let vec = V3::make((y*kz-ky*z, -(x*kz-kx*z), x*ky-kx*y));
            vec.modl()
        }
    }
}