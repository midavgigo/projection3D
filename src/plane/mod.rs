pub mod plane{
    use tmn::Nums;
    use tmn::quaternion::QNum;
    use crate::line::line::Line;
    use crate::vector::vector::V3;

    pub struct Plane{
        coef:Nums
    }

    impl Plane{
//Конструктор плоскости через 3 точки, пока не планируется для использования
/*        pub fn make(a:(f32, f32, f32), b:(f32, f32, f32), c:(f32, f32, f32)) -> Self{
            let (x1, y1, z1) = a;
            let (x2, y2, z2) = b;
            let (x3, y3, z3) = c;
            let qnum = QNum::make_from_r((y2-y1)*(z3-z1)-(y3-y1)*(z2-z1),
                                                (z2-z1)*(x3-x1)-(z3-z1)*(x2-x1),
                                                (x2-x1)*(y3-y1)-(x3-x1)*(y2-y1),
                                                z1*(x3-x1)*(y2-y1)+x1*(y3-y1)*(z2-z1)+y1*(z3-z1)*(x2-x1)-x1*(y2-y1)*(z3-z1)-y1*(z2-z1)*(x3-x1)-z1*(x2-x1)*(y3-y1));
            Self{
                coef:Nums::Quaternion(qnum)
            }
        }*/
        pub fn mk_pvec(dir: &V3, pos:&V3) -> Self{
            let (x, y, z) = dir.get();
            let (x0, y0, z0) = dir.add(pos).get();
            let qnum = QNum::make_from_r(x, y, z, -(x*x0+y*y0+z*z0));
            Self{
                coef:Nums::Quaternion(qnum)
            }
        }
        //Возвращает параметр пересечения
        fn get_t(&self, ln:&Line) -> f32{
            let (x1, y1, z1) = ln.get_pos().get();
            let (kx, ky, kz) = ln.get_vec().get();
            if let Nums::Quaternion(qnum) = self.coef.clone(){
                let (a, b, c, d) = qnum.get();
                -(a*x1+b*y1+c*z1+d)/(a*kx+b*ky+c*kz)
            }else{
                panic!("Wrong type of coef");
            }
        }

        pub fn intersect_with_line(&self, ln:&Line) -> V3{
            let t = self.get_t(ln);
            let (x1, y1, z1) = ln.get_pos().get();
            let (kx, ky, kz) = ln.get_vec().get();
            V3::make((kx*t+x1, ky*t+y1, kz*t+z1))
        }
    }
}