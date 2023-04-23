pub mod object{
    use crate::vector::vector::V3;

    pub struct Obj{
        polygons: Vec<(usize, usize, usize)>,
        points: Vec<V3>,
        pos: V3
    }

    impl Obj<>{
        pub fn make(points:&Vec<V3>, refs:&Vec<(usize, usize, usize)>, pos:&V3) -> Self{
            let mut pts = Vec::new();
            for i in points{
                pts.push(i.clone());
            }
            let mut rfs = Vec::new();
            for i in refs{
                rfs.push(*i);
            }
            Self{
                polygons: rfs,
                points:pts,
                pos:pos.clone()
            }
        }

        pub fn get_polygons(&self)->Vec<(V3, V3, V3)>{
            let mut ret = Vec::new();
            let pos = &self.pos;
            for i in &self.polygons{
                ret.push((self.points[i.0].add(pos), self.points[i.1].add(pos), self.points[i.2].add(pos)));
            }
            ret
        }

        pub fn get_refs(&self)->Vec<(usize, usize, usize)>{
            self.polygons.clone()
        }

        pub fn get_points(&self)->Vec<V3>{
            let mut ret = Vec::new();
            for i in &self.points{
                ret.push(i.clone());
            }
            ret
        }

        pub fn set_points(&mut self, ps:&Vec<V3>){
            for i in 0..self.points.len(){
                self.points[i] = ps[i].clone();
            }
        }

        pub fn rotate(&mut self, ang:f32, o:&V3){
            for i in &mut self.points {
                i.rotate(ang, o);
            }
        }
        pub fn get_pos(&self) -> &V3{
            &self.pos
        }
        pub fn set_pos(&mut self, pos:&V3){
            self.pos = pos.clone();
        }

        pub fn clone(&self) -> Self{
            let mut points = Vec::new();
            for i in &self.points{
                points.push(i.clone());
            }
            Self{
                polygons:self.polygons.clone(),
                points,
                pos:self.pos.clone()
            }
        }
    }
}