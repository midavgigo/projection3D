pub(crate) mod vars;

pub mod animation{
    use crate::filer;
    use crate::filer::filer::{f32_to_u8, u32_to_f32, u8_to3_u32, u8_to_u32};
    use crate::object::object::Obj;
    use crate::vector::vector::V3;

    pub struct Command{
        cmd:u8,
        args:Vec<u8>
    }

    impl Command{
        pub fn make(cmd:u8, args:&Vec<u8>)->Self{
            Self{
                cmd,
                args: args.clone()
            }
        }
        pub fn get_debug(&self)->(u8, &Vec<u8>){
            (self.cmd, &self.args)
        }
    }

    pub struct Animation<>{
        marks: [u8; 256],
        cmds: Vec<Command>,
        buffer: [u32; 256],
        obj: Obj,
        step: u8
    }
    impl Animation<>{
        pub fn make<>(path:&str, obj: Obj)->Self{
            let get = filer::filer::read_animation(path).expect("Не удалось прочитать файл анимации");
            let marks = get.0;
            let cmds = get.1;
            Animation{
                marks,
                cmds,
                buffer: [0; 256],
                obj,
                step:0
            }
        }
        pub fn get_debug(&self)->(&[u8], &Vec<Command>){
            (&self.marks[..], &self.cmds)
        }

        pub fn get_obj(&self)->&Obj{
            &self.obj
        }

        fn add(&mut self, i:u8, v:u32){
            self.buffer[i as usize] = (self.buffer[i as usize] as u64 + v as u64) as u32;
        }
        fn buffer(&mut self, i:u8, v:u32){
            self.buffer[i as usize] =  v;
        }
        fn check(&self, i:u8, v:u32, m1:u8, m2:u8)->u8{
            if self.buffer[i as usize] == v{
                m1
            }else{
                m2
            }
        }
        fn move_p(&mut self, i:u8, l:u8, x:f32, y:f32, z:f32){
            let mut points = self.obj.get_points();
            let slc = &self.buffer[(i as usize)..((i+l) as usize)];
            let shift = V3::make((x, y, z));
            for i in slc{
                if *i == 0{
                    self.obj.set_pos(&self.obj.get_pos().add(&shift));
                }else{
                    let i = *i-1;
                    points[i as usize] = points[i as usize].add(&shift);
                }
            }
            self.obj.set_points(&points);
        }
        fn rotate(&mut self, i:u8, l:u8, a:f32, x:f32, y:f32, z:f32){
            let slc = &self.buffer[(i as usize)..((i+l) as usize)];
            let mut pos = self.obj.get_pos().clone();
            let mut points = self.obj.get_points();
            for i in slc{
                if *i == 0{
                    pos.rotate(a, &V3::make((x, y, z)));
                    self.obj.set_pos(&pos);
                }else{
                    let i = *i-1;
                    points[i as usize].rotate(a, &V3::make((x, y, z)));
                }
            }
            self.obj.set_points(&points);
        }
        fn set(&mut self, i:u8, l:u8, x:f32, y:f32, z:f32){
            let mut points = self.obj.get_points();
            let slc = &self.buffer[(i as usize)..((i+l) as usize)];
            for i in slc{
                if *i == 0{
                    self.obj.set_pos(&V3::make((x, y, z)));
                }else{
                    let i = *i-1;
                    points[i as usize] = V3::make((x, y, z));
                }
            }
            self.obj.set_points(&points);
        }
        fn write(&mut self, p:u8, i:u8){
            let mut dot = V3::make((0.0, 0.0, 0.0));
            if p == 0{
                dot = self.obj.get_pos().clone();
            }else {
                let p = p-1;
                dot = self.obj.get_points()[p as usize].clone();
            }
            let points = [dot.get().0, dot.get().1, dot.get().2];
            for j in 0..3{
                let slc = f32_to_u8(points[j]);
                self.buffer[i as usize+j]+=slc[3] as u32;
                self.buffer[i as usize+j]+=(slc[2] as u32 )<<8;
                self.buffer[i as usize+j]+=(slc[1] as u32 )<<16;
                self.buffer[i as usize+j]+=(slc[0] as u32)<<24;
            }
        }

        fn twist(&mut self, i:u8, l:u8, ang:f32, p1:u8, p2:u8){
            let mut points = self.obj.get_points();
            let slc = &self.buffer[(i as usize)..((i+l) as usize)];
            let x = u32_to_f32(self.buffer[p1 as usize]);
            let y = u32_to_f32(self.buffer[p1 as usize + 1]);
            let z = u32_to_f32(self.buffer[p1 as usize + 2]);
            let a1 = V3::make((x, y, z));
            let x = u32_to_f32(self.buffer[p2 as usize]);
            let y = u32_to_f32(self.buffer[p2 as usize + 1]);
            let z = u32_to_f32(self.buffer[p2 as usize + 2]);
            let a2 = V3::make((x, y, z));
            let vc = a1.add(&a2.neg());
            let (a, b, c) = vc.get();
            let (x1, y1, z1) = a1.get();
            for i in slc{
                let (mut x0, mut y0, mut z0) = (0.0, 0.0, 0.0);
                if *i == 0{
                    (x0, y0, z0) = self.obj.get_pos().get();
                }else{
                    let i = *i-1;
                    (x0, y0, z0) = points[i as usize].get();
                }
                let t = (a*x0 + b*y0 + c*z0);
                let t = (t - x1*a - y1*b - z1*c)/(a*a + b*b + c*c);
                let p = V3::make((t*a+x1, t*b+y1, t*c+z1));
                let mut rv = V3::make((0.0, 0.0, 0.0));
                if *i == 0{
                    rv = self.obj.get_pos().add(&p.neg());
                    rv.rotate(ang, &vc);
                    self.obj.set_pos(&p.add(&rv));
                }else{
                    let i = *i-1;
                    rv = points[i as usize].add(&p.neg());
                    rv.rotate(ang, &vc);
                    points[i as usize] = p.add(&rv);
                }
            }
            self.obj.set_points(&points);
        }

        fn product(&mut self, i:u8, q:f32){
            let slc = &self.buffer[(i as usize)..(i as usize +3)];
            let x = slc[0];
            let y = slc[1];
            let z = slc[2];
            let x = u32_to_f32(x)*q;
            let y = u32_to_f32(y)*q;
            let z = u32_to_f32(z)*q;
        }

        pub fn iterate(&mut self){
            if self.is_end(){return;}
            let cmd = &self.cmds[self.step as usize];
            //println!("{:?}", self.buffer);
            let mark = match cmd.cmd {
                    65=>{
                        let mut num:u32 = u8_to_u32(&cmd.args[1..5]);
                        self.add(cmd.args[0], num);
                        None
                    },
                    66=>{
                        let mut num:u32 = u8_to_u32(&cmd.args[1..5]);
                        self.buffer(cmd.args[0], num);
                        None
                    },
                    67=>{
                        let mut num:u32 = u8_to_u32(&cmd.args[1..5]);
                        Some(self.check(cmd.args[0], num, cmd.args[5], cmd.args[6]))
                    },
                    74=>{
                        Some(cmd.args[0])
                    },
                    77=>{
                        let (x, y, z) = u8_to3_u32(&cmd.args[2..14]);
                        let (x, y, z) = (u32_to_f32(x), u32_to_f32(y), u32_to_f32(z));
                        self.move_p(cmd.args[0], cmd.args[1], x, y, z);
                        None
                    },
                    81=>{
                        let q = u8_to_u32(&cmd.args[1..5]);
                        let q = u32_to_f32(q);
                        self.product(cmd.args[0], q);
                        None
                    },
                    82=>{
                        let (a, x, y) = u8_to3_u32(&cmd.args[2..14]);
                        let z = u8_to_u32( &cmd.args[14..18]);
                        let (a, x, y, z) = (u32_to_f32(a), u32_to_f32(x), u32_to_f32(y), u32_to_f32(z));
                        self.rotate(cmd.args[0], cmd.args[1], a, x, y, z);
                        None
                    },
                    83=>{
                        let (x, y, z) = u8_to3_u32(&cmd.args[2..14]);
                        let (x, y, z) = (u32_to_f32(x), u32_to_f32(y), u32_to_f32(z));
                        self.set(cmd.args[0], cmd.args[1], x, y, z);
                        None
                    },
                    84=>{
                        let mut ang:u32 = u8_to_u32(&cmd.args[2..6]);
                        self.twist(cmd.args[0], cmd.args[1], u32_to_f32(ang), cmd.args[6], cmd.args[7]);
                        None
                    },
                    87=>{
                        self.write(cmd.args[0], cmd.args[1]);
                        None
                    },
                    97=>{
                        self.add(self.buffer[cmd.args[0] as usize] as u8, self.buffer[cmd.args[1] as usize]);
                        None
                    },
                    98=>{
                        self.buffer(self.buffer[cmd.args[0] as usize] as u8, self.buffer[cmd.args[1] as usize]);
                        None
                    },
                    99=>{Some(self.check(self.buffer[cmd.args[0] as usize] as u8, self.buffer[cmd.args[1] as usize] as u32, self.buffer[cmd.args[2] as usize] as u8, self.buffer[cmd.args[3] as usize] as u8))},
                    106=>{Some(self.buffer[cmd.args[0] as usize] as u8)},
                    109=>{
                        self.move_p(self.buffer[cmd.args[0] as usize] as u8, self.buffer[cmd.args[1] as usize] as u8, u32_to_f32(self.buffer[cmd.args[2] as usize]), u32_to_f32(self.buffer[cmd.args[3] as usize]), u32_to_f32(self.buffer[cmd.args[4] as usize]));
                        None
                    },
                    114=>{
                        self.rotate(self.buffer[cmd.args[0] as usize] as u8, self.buffer[cmd.args[1] as usize] as u8, u32_to_f32(self.buffer[cmd.args[2] as usize]), u32_to_f32(self.buffer[cmd.args[3] as usize]), u32_to_f32(self.buffer[cmd.args[4] as usize]), u32_to_f32(self.buffer[cmd.args[5] as usize]));
                        None
                    },
                    115=>{
                        self.set(self.buffer[cmd.args[0] as usize] as u8, self.buffer[cmd.args[1] as usize] as u8, u32_to_f32(self.buffer[cmd.args[2] as usize]), u32_to_f32(self.buffer[cmd.args[3] as usize]), u32_to_f32(self.buffer[cmd.args[4] as usize]));
                        None
                    }
                    _=>None
                };
            match mark{
                Some(m)=>{
                    if m != 0{ self.step = self.marks[m as usize];}
                    else { self.step = self.cmds.len() as u8;}
                },
                None=>{
                    self.step+=1;
                }
            }
        }
        pub fn is_end(&self)->bool{
            self.step as usize == self.cmds.len()
        }
    }
}