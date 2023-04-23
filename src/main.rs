extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::animation::animation::Animation;

use crate::camera::camera::Camera;
use crate::filer::filer::read_model;
use crate::filer::filer::{f32_to_u8, u8_to_u32, u32_to_f32};
use crate::vector::vector::V3;

mod camera;
mod vector;
mod plane;
mod line;
mod object;
mod filer;
mod animation;

const WID:u32 = 800;
const HEI:u32 = 800;

fn main() {
    start_game("test_twist.ap3d");
}


fn start_game(anim_path:&str){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", WID, HEI)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut camera = Camera::make(&V3::make((-10.0,0.0,1.0)), 1.0, 1.0);
    let stab = camera.get_axis().2.clone();
    let mut shlepa = read_model("shlepa.p3d").expect("Не могу прочитать файл");
    shlepa.rotate(180.0, &stab);
    shlepa.set_pos(&V3::make((0.0, 0.0, 1.0)));
    let mut anim = Animation::make(anim_path, shlepa);
    let mut w:i8 = 0;
    let mut n:i8 = 0;
    'running: loop {
        canvas.set_draw_color(Color::RGB(15, 150, 25));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion { xrel, yrel, ..}=>{
                    if xrel!=0{
                        let stab = V3::make((0.0, 0.0, -(xrel/xrel.abs()) as f32));
                        let axis = stab;
                        camera.rotate(1.0, &axis);
                        //sdl_context.mouse().warp_mouse_in_window(&canvas.window(), (WID>>1) as i32, (HEI>>1) as i32);
                    }
                    if yrel!=0{
                        let axis = camera.get_axis().1.clone().mul((yrel / yrel.abs()) as f32);
                        camera.rotate(1.0, &axis);
                        if camera.get_axis().2.get().2 <= 0.0{
                            camera.rotate(-1.0, &axis);
                        }
                        //sdl_context.mouse().warp_mouse_in_window(&canvas.window(), (WID>>1) as i32, (HEI>>1) as i32);
                    }
                },
                Event::KeyDown { keycode,  ..}=>{
                    if let Some(Keycode::W) = keycode{
                        n = 1;
                    }
                    if let Some(Keycode::S) = keycode{
                        n = -1;
                    }
                    if let Some(Keycode::A) = keycode{
                        w = 1;
                    }
                    if let Some(Keycode::D) = keycode{
                        w = -1;
                    }
                    if let Some(Keycode::Space) = keycode{
                        camera.set_pos(&camera.get_pos().add(&V3::make((0.0, 0.0, 0.1))));
                    }
                    if let Some(Keycode::C) = keycode{
                        camera.set_pos(&camera.get_pos().add(&V3::make((0.0, 0.0, -0.1))));
                    }
                },
                Event::KeyUp { keycode,  ..}=>{
                    if let Some(Keycode::W) = keycode{
                        n = 0;
                    }
                    if let Some(Keycode::S) = keycode{
                        n = 0;
                    }
                    if let Some(Keycode::A) = keycode{
                        w = 0;
                    }
                    if let Some(Keycode::D) = keycode{
                        w = 0;
                    }
                },
                _ => {}
            }
        }
        let mut nose = camera.get_axis().0.clone();
        let mut wing = camera.get_axis().1.clone();
        let stab = camera.get_axis().2.clone();
        if nose.modl()>0.0{
            let q = (nose.get().0.powi(2) +nose.get().1.powi(2)).powf(0.5);
            nose = V3::make((nose.get().0, nose.get().1, 0.0)).mul((n as f32) * 0.01 / q);
        }else{
            let q = (stab.get().0.powi(2) +stab.get().1.powi(2)).powf(0.5);
            nose = V3::make((-stab.get().0, -stab.get().1, 0.0)).mul((n as f32) * 0.01 / q);
        }
        wing = V3::make((wing.get().0, wing.get().1, 0.0)).mul((w as f32)*0.01);
        camera.set_pos(&camera.get_pos().add(&nose.add(&wing)));

        canvas.set_draw_color(Color::RGB(255,255,255));
        for (q, w, e) in anim.get_obj().get_polygons(){
            draw_polygon(&q, &w, &e, &mut canvas, &camera);
        }
        /*let tmp = &anim.get_obj().get_points()[zhopa].add(anim.get_obj().get_pos());
        let xy = camera.projection_dot(tmp);
        let xy = get_point(xy, &camera);
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        if let Some((x, y)) = xy{ canvas.fill_rect(Rect::new(x - 5, y - 5, 10, 10)).expect("TODO: panic message"); }
        */
        canvas.present();
        anim.iterate();
    }
}

fn get_point(o:Option<(f32, f32)>, camera:&Camera)->Option<(i32, i32)>{
    let Some((x, y)) = o else{return None;};
    Some(camera.utos(x, y, WID, HEI))
}

fn draw_polygon(q:&V3, w:&V3, e:&V3, canvas: &mut WindowCanvas, camera:&Camera){
    let (a, b, c, d) = camera.projection_polygon(&q, &w, &e);
    let (a_, b_, c_, d_) = (get_point(a, camera), get_point(b, camera), get_point(c, camera), get_point(d, camera));
    if let Some((x1, y1)) = a_ {
        if let Some((x2, y2)) = b_ {
            if let Some((x3, y3)) = c_ {
                if let Some((x4, y4)) = d_ {
                    canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2)).expect("Шота не так");
                    canvas.draw_line(Point::new(x2, y2), Point::new(x4, y4)).expect("Шота не так");
                    canvas.draw_line(Point::new(x4, y4), Point::new(x3, y3)).expect("Шота не так");
                    canvas.draw_line(Point::new(x3, y3), Point::new(x1, y1)).expect("Шота не так");
                } else {
                    canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2)).expect("Шота не так");
                    canvas.draw_line(Point::new(x2, y2), Point::new(x3, y3)).expect("Шота не так");
                    canvas.draw_line(Point::new(x3, y3), Point::new(x1, y1)).expect("Шота не так");
                }
            }
        }
    }
}