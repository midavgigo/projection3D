pub mod camera{
    use crate::line::line::Line;
    use crate::plane::plane::Plane;
    use crate::vector::vector::V3;

    pub const NOSE:u8 = 1;
    pub const WING:u8 = 2;
    pub const STAB:u8 = 4;

    pub struct Camera{
        nose:V3,
        wing:V3,
        stab:V3,
        pos:V3,
        c:V3,
        u:V3,
        l:V3,
        screen:Plane,
        size:(f32, f32)
    }

    impl Camera {
        pub fn make(pos:&V3, height:f32, width:f32) -> Self{
            let (nose,wing,stab) = (V3::make((1.0, 0.0, 0.0)),
                                      V3::make((0.0, 1.0, 0.0)),
                                      V3::make((0.0, 0.0, 1.0)));
            let l = pos.add(&nose).add(&wing.mul(width /wing.modl()));
            let u = pos.add(&nose).add(&stab.mul(height /stab.modl()));
            let c = pos.add(&nose);
            let pln = Plane::mk_pvec(&nose, &pos.clone());
            Self{
                nose:nose.clone(),
                wing:wing.clone(),
                stab:stab.clone(),
                pos:pos.clone(),
                c,
                u,
                l,
                screen:pln,
                size:(width, height)
            }
        }

        pub fn projection_dot(&self, dot:&V3) -> Option<(f32, f32)> {
            if dot.add(&self.pos.add(&self.nose).neg()).cos(&self.nose.neg()) > 0.0{
                return None;
            }
            let a = self.screen.intersect_with_line(&Line::mk_2dots(&dot, &self.pos));
            //println!("{} {} {}", a.get().0, a.get().1, a.get().2);
            let h = Line::mk_2dots(&self.l, &self.c).distance(&a)/self.size.0;
            let w = Line::mk_2dots(&self.u, &self.c).distance(&a)/self.size.1;
            //println!("{h} {w}");
            if (0.0, 0.0) == (h, w){
                return Some((0.0,0.0));
            }
            let cu = self.u.clone().add(&self.c.neg());
            let cl = self.l.clone().add(&self.c.neg());
            let ca = a.add(&self.c.neg());
            let qh = if cu.cos(&ca) == 0.0 {1.0}else{cu.cos(&ca)/cu.cos(&ca).abs()};
            let qw = if cl.cos(&ca) == 0.0 {1.0}else{cl.cos(&ca)/cl.cos(&ca).abs()};
            //println!("{} {}",-w*qw ,h*qh);
            Some((-w*qw, h*qh))
        }

        fn get_t(&self, ln:&Line) -> f32{//Параметр видимости
            let (x0, y0, z0) = self.pos.add(&self.nose).get();
            let (x1, y1, z1) = ln.get_pos().get();
            let (kx, ky, kz) = ln.get_vec().get();
            let (xd, yd, zd) = self.nose.get();
            let n = - (x1-x0)*xd - (y1-y0)*yd - (z1 - z0)*zd;
            let d = kx*xd + ky*yd + kz*zd;
            n/d
        }

        fn get_dot(ln:&Line, t:f32) -> V3{//точка из параметра линии
            let (x1, y1, z1) = ln.get_pos().get();
            let (kx, ky, kz) = ln.get_vec().get();
            let ret = V3::make((kx*t+x1, ky*t+y1, kz*t+z1));
            ret
        }


        pub fn projection_segment(&self, point_a: &V3, point_b: &V3) -> (Option<(f32, f32)>, Option<(f32, f32)>){
            if let None = self.projection_dot(point_a){
                if let None = self.projection_dot(point_b){
                    (None, None)
                }else{
                    let ln = Line::mk_2dots(point_a, point_b);
                    let t:f32 = self.get_t(&ln);
                    let a = self.projection_dot(&Self::get_dot(&ln, t));
                    (a, self.projection_dot(point_b))
                }
            }else{
                if let None = self.projection_dot(point_b){
                    let ln = Line::mk_2dots(point_a, point_b);
                    let t:f32 = self.get_t(&ln);
                    let b = self.projection_dot(&Self::get_dot(&ln, t));
                    (self.projection_dot(point_a), b)
                }else{
                    (self.projection_dot(point_a), self.projection_dot(point_b))
                }
            }
        }

        pub fn sort(a:&Option<(f32, f32)>, b:&Option<(f32, f32)>, c:&Option<(f32, f32)>, aa:&V3, bb:&V3, cc:&V3) ->(Option<(f32, f32)>, Option<(f32, f32)>, Option<(f32, f32)>, V3, V3, V3){
            let mut mas = [a, b, c];
            let mut ret = [aa, bb, cc];
            for i in (0..3).rev(){
                for j in 0..i{
                    if let None = mas[j+1]{
                        let temp = mas[j+1];
                        mas[j+1] = mas[j];
                        mas[j] = temp;
                        let temp = ret[j+1];
                        ret[j+1] = ret[j];
                        ret[j] = temp;
                    }
                }
            }
            (*mas[0], *mas[1], *mas[2], ret[0].clone(), ret[1].clone(), ret[2].clone())
        }

        pub fn projection_polygon(&self, point_a:&V3, point_b:&V3, point_c:&V3) -> (Option<(f32, f32)>, Option<(f32, f32)>, Option<(f32, f32)>, Option<(f32, f32)>){
            let a_pr = self.projection_dot(point_a);
            let b_pr = self.projection_dot(point_b);
            let c_pr = self.projection_dot(point_c);
            let (t1, t2, t3, v1, v2, v3) = Self::sort(&a_pr, &b_pr, &c_pr, &point_a, &point_b, &point_c);
            return if let None = t1 {
                if let None = t2 {
                    if let None = t3 {//все точки за экраном
                        (None, None, None, None)
                    } else {//две точки за экраном
                        let (_t, t2) = self.projection_segment(&v3, &v2);
                        let (_t, t1) = self.projection_segment(&v3, &v1);
                        (t1, t2, t3, None)
                    }
                } else {//одна точка за экраном
                    let (_t, t12) = self.projection_segment(&v2, &v1);
                    let (_t, t13) = self.projection_segment(&v3, &v1);
                    (t12, t13, t2, t3)
                }
            } else {//все точки перед экраном
                (t1, t2, t3, None)
            }
        }
        pub fn rotate(&mut self, ang:f32, o:&V3){
            self.nose.rotate(ang, o);
            self.wing.rotate(ang, o);
            self.stab.rotate(ang, o);

            self.l = self.pos.add(&self.nose).add(&self.wing.mul(self.l.add(&self.c.neg()).modl() / self.wing.modl()));
            self.u = self.pos.add(&self.nose).add(&self.stab.mul(self.u.add(&self.c.neg()).modl() / self.stab.modl()));
            self.c = self.pos.add(&self.nose);
            self.screen = Plane::mk_pvec(&self.nose, &self.pos.clone());
        }

        pub fn get_pos(&self)->&V3{ &self.pos }

        pub fn get_axis(&self)->(&V3, &V3, &V3){ (&self.nose, &self.wing, &self.stab) }

        pub fn set_pos(&mut self, pos:&V3){
            self.pos = pos.clone();
            self.l = self.pos.add(&self.nose).add(&self.wing.mul(self.l.add(&self.c.neg()).modl() / self.wing.modl()));
            self.u = self.pos.add(&self.nose).add(&self.stab.mul(self.u.add(&self.c.neg()).modl() / self.stab.modl()));
            self.c = self.pos.add(&self.nose);
            self.screen = Plane::mk_pvec(&self.nose, &self.pos.clone());
        }

        pub fn utos(&self, x:f32, y:f32, wid:u32, hei:u32)->(i32, i32){
            let (wid, hei) = ((wid>>1) as f32, (hei>>1) as f32);
            ((wid*(1.0+x)) as i32, (hei*(1.0-y)) as i32)
        }

        pub fn stou(&self, wid:u32, hei:u32, x:i32, y:i32) ->(f32, f32){
            let (wid, hei) = (wid >> 1, hei >> 1);
            ((x - wid as i32) as f32/wid as f32, (y - hei as i32) as f32/hei as f32)
        }

        pub fn get_axis_on_screen(&self, x:f32, y:f32) -> V3{
            let mut ret = self.wing.mul(x).add(&self.stab.mul(y));
            ret.rotate(90.0, &self.nose);
            ret
        }
    }
}