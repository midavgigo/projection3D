pub mod filer{
    use std::fs::File;
    use std::io::{Write, Error};
    use std::{fs, mem};
    use crate::animation::animation::{Command};
    use crate::animation::vars::vars::Commands;
    use crate::object::object::Obj;
    use crate::vector::vector::V3;

    pub fn f32_to_u8(inp:f32)->[u8; 4]{
        let conv = inp.to_bits();
        let mut ret:[u8; 4] = [0;4];
        let mut mask:u32 = 4278190080;
        let mut shift:i8 = 24;
        for i in 0..4{
            ret[i] = ((conv & mask)>>shift)as u8;
            mask >>= 8;
            shift -= 8;
        }
        ret
    }
    pub fn u32_to_f32(inp:u32)->f32{
        unsafe {
            mem::transmute::<u32, f32>(inp) as f32
        }
    }

    pub fn u8_to_u32(inp:&[u8])->u32{
        let mut ret:u32 = 0;
        for i in 0..4{
            ret += (inp[i] as u32)<<8*(3-i);
        }
        ret
    }
    pub fn u8_to3_u32(inp:&[u8])->(u32, u32, u32){
        let ret = (
            u8_to_u32(&inp[0..4]),
            u8_to_u32(&inp[4..8]),
            u8_to_u32(&inp[8..12]));
        ret
    }

    pub fn write_model(path:&str, ob:&Obj) -> Result<(), Error>{
        let mut output = File::create(path)?;
        let points = ob.get_points();
        let nd = points.len() as u32 + 1;
        output.write_all(nd.to_be_bytes().as_slice()).expect("Ошибка в записи количества точек");
        let (x, y, z) = ob.get_pos().get();
        output.write_all(&f32_to_u8(x)).expect("Ошибка в записи точки");
        output.write_all(&f32_to_u8(y)).expect("Ошибка в записи точки");
        output.write_all(&f32_to_u8(z)).expect("Ошибка в записи точки");
        for i in points{
            let (x, y, z) = i.get();
            output.write_all(&f32_to_u8(x)).expect("Ошибка в записи точки");
            output.write_all(&f32_to_u8(y)).expect("Ошибка в записи точки");
            output.write_all(&f32_to_u8(z)).expect("Ошибка в записи точки");
        }
        for i in ob.get_refs(){
            let (x, y, z) = i;
            output.write_all((x as u32).to_be_bytes().as_slice()).expect("Ошибка в записи точки полигона");
            output.write_all((y as u32).to_be_bytes().as_slice()).expect("Ошибка в записи точки полигона");
            output.write_all((z as u32).to_be_bytes().as_slice()).expect("Ошибка в записи точки полигона");
        }
        Ok(())
    }
    pub fn read_model(path:&str)->Result<Obj, Error>{
        let u8s = fs::read(path).expect("Ошибка в чтении файла");
        let nd:u32 = ((u8s[0] as u32)<<24) + ((u8s[1] as u32) << 16) + ((u8s[2] as u32) << 8) + (u8s[3] as u32);
        let mut points = Vec::new();
        let mut pos = V3::make((0.0, 0.0, 0.0));
        let mut beg:usize = 4;
        for i in 0..nd{
            let x:f32 = u32_to_f32(((u8s[beg] as u32)<<24) + ((u8s[beg+1] as u32) << 16) + ((u8s[beg+2] as u32) << 8) + (u8s[beg+3] as u32));
            let y:f32 = u32_to_f32(((u8s[beg+4] as u32)<<24) + ((u8s[beg+5] as u32) << 16) + ((u8s[beg+6] as u32) << 8) + (u8s[beg+7] as u32));
            let z:f32 = u32_to_f32(((u8s[beg+8] as u32)<<24) + ((u8s[beg+9] as u32) << 16) + ((u8s[beg+10] as u32) << 8) + (u8s[beg+11] as u32));
            if i != 0{
                points.push(V3::make((x, y, z)));
            }else{
                pos = V3::make((x, y, z));
            }
            beg+=12;
        }
        let mut rfs:Vec<(usize, usize, usize)> = Vec::new();
        for i in (beg..u8s.len()).step_by(12){
            let x:u32 = ((u8s[i] as u32)<<24) + ((u8s[i+1] as u32) << 16) + ((u8s[i+2] as u32) << 8) + (u8s[i+3] as u32);
            let y:u32 = ((u8s[i+4] as u32)<<24) + ((u8s[i+5] as u32) << 16) + ((u8s[i+6] as u32) << 8) + (u8s[i+7] as u32);
            let z:u32 = ((u8s[i+8] as u32)<<24) + ((u8s[i+9] as u32) << 16) + ((u8s[i+10] as u32) << 8) + (u8s[i+11] as u32);
            rfs.push((x as usize, y as usize, z as usize));
        }
        let ob = Obj::make(&points, &rfs, &pos);
        Ok(ob)
    }
    pub fn read_animation(path:&str)->Result<([u8;256], Vec<Command>), Error>{
        let u8s = fs::read(path).expect("Ошибка в чтении анимации");
        let mut marks:[u8; 256] = [0; 256];
        let mut cmds:Vec<Command> = Vec::new();
        let mut i = 0;
        let commands = Commands::init();
        let commands_args = commands.get_commands();
        while i<u8s.len(){
            let mark = *u8s.get(i).expect("Ошибка в чтении");
            marks[mark as usize] = cmds.len() as u8;
            i+=1;
            let cmd = *u8s.get(i).expect("Ошибка в чтении");
            i+=1;
            //println!("{cmd}");
            let arg_sizes = commands_args.get(&cmd).expect("Ошибка в инициализировании команды");
            let mut args:Vec<u8> = Vec::new();
            for j in arg_sizes{
                for _j in 0..*j{
                    args.push(*u8s.get(i).expect("Ошибка в чтении"));
                    i+=1;
                }
            }
            cmds.push(Command::make(cmd, &args));
        }
        Ok((marks, cmds))
    }
}