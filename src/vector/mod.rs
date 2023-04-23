pub mod vector{
    use tmn::Nums;
    use tmn::quaternion::QNum;

    pub struct V3{
        qnum:Nums
    }
    impl V3{
        pub fn make(v:(f32, f32, f32)) -> Self{
            let qnum = Nums::Quaternion(QNum::make_from_r(0.0, v.0, v.1, v.2));
            Self{
                qnum
            }
        }
        pub fn get(&self)->(f32, f32, f32){
            let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
            if let Nums::Quaternion(q)=&self.qnum{
                let (_r, i, j, k) = q.get();
                x = i;
                y = j;
                z = k;
            }else{
                panic!("Something wrong with typing of Nums");
            }
            (x, y, z)
        }
        pub fn rotate(&mut self, ang:f32, o:&V3){ self.qnum = self.qnum.rot(ang, o.get()); }
        pub fn clone(&self) -> Self{
            Self{
                qnum:self.qnum.clone()
            }
        }
        pub fn modl(&self) -> f32{
            if let Nums::Quaternion(qnum) = self.qnum.clone(){
                qnum.modl()
            }else{
                panic!("Wrong type of vector");
            }
        }

        pub fn mul(&self, v:f32) -> Self{
            Self{
                qnum:self.qnum.clone()*Nums::Real(v)
            }
        }

        pub fn cos(&self, v:&V3) -> f32{
            let (x1, y1, z1) = self.get();
            let (x2, y2, z2) = v.get();
            let num = x1*x2+y1*y2+z1*z2;
            num/(self.modl()*v.modl())
        }
        pub fn add(&self, rhs: &Self) -> Self{
            let mut qnum = self.clone().qnum+rhs.clone().qnum;
            qnum = qnum.set(tmn::quaternion::R, 0.0);
            Self{
                qnum
            }
        }
        pub fn neg(&self) -> Self{
            Self {
                qnum:-self.clone().qnum
            }
        }
    }
}