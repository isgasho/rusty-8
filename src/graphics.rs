use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::collections::HashSet;

//mod graphics;

extern crate sdl2;

pub struct Graphics {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
//    window: sdl2::video::Window,
    renderer: sdl2::render::Renderer<'static>,
    event_pump: sdl2::EventPump
}

impl Default for Graphics {
    #[inline]
    fn default() -> Graphics {
        let sdl_con = sdl2::init()
            .unwrap();

        let video_sub = sdl_con
            .video()
            .unwrap();
        
        let win = video_sub
            .window("Chip-8 emulator", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        
        let mut rend = win
            .renderer()
            .build()
            .unwrap();

        rend.set_draw_color(Color::RGB(0,0,0));
        rend.set_scale(8.0,8.0); //check this later
        rend.clear();
        rend.present();

        let event_p = sdl_con
            .event_pump()
            .unwrap();

        Graphics {
            sdl_context:     sdl_con,
            video_subsystem: video_sub,
            renderer:        rend,
            event_pump:      event_p
        }
        
    }
}


impl Graphics {
    pub fn graphics(&mut self) {

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),..}
                    => {
                        break 'running
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn is_pressed(&mut self, key: u8) -> bool
    {
        let mut kc: Keycode;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode:
                                 Some(Keycode::Num0),.. }
                => { kc = Keycode::Num0 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num1),.. }
                => { kc = Keycode::Num1 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num2),.. }
                => { kc = Keycode::Num2 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num3),.. }
                => { kc = Keycode::Num3 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num4),.. }
                => { kc = Keycode::Num4 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num5),.. }
                => { kc = Keycode::Num5 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num6),.. }
                => { kc = Keycode::Num6 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num7),.. }
                => { kc = Keycode::Num7 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num8),.. }
                => { kc = Keycode::Num8 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num9),.. }
                => { kc = Keycode::Num9 },
                Event::KeyDown { keycode:
                                 Some(Keycode::A),.. }
                => { kc = Keycode::A },
                Event::KeyDown { keycode:
                                 Some(Keycode::B),.. }
                => { kc = Keycode::B },
                Event::KeyDown { keycode:
                                 Some(Keycode::C),.. }
                => { kc = Keycode::C },
                Event::KeyDown { keycode:
                                 Some(Keycode::D),.. }
                => { kc = Keycode::D },
                Event::KeyDown { keycode:
                                 Some(Keycode::E),.. }
                => { kc = Keycode::E },
                Event::KeyDown { keycode:
                                 Some(Keycode::F),.. }
                => { kc = Keycode::F },
                _ => { /* do nothing */
                kc = Keycode::G}

            }
            if kc == lookup_keycode(key)
            {   return true; }
            
        }
            false
    }


    pub fn draw_point(&mut self, screen: [[bool; 64]; 32]) {
        self.renderer.set_draw_color(
            Color::RGB(255,255,255));
        for i in 0..64 {
            for j in 0..32 {
                if screen[j][i] {
                    self.renderer.draw_point(
                        sdl2::rect::Point::new(i as i32,j as i32) );

                }
            }
        }
        //self.renderer.draw_point(
        //    sdl2::rect::Point::new(x,y) );
    }

    pub fn draw_screen(&mut self) {
        self.renderer.present();
    }

    pub fn clear_screen(&mut self) {
        self.renderer.set_draw_color(
            Color::RGB(0,0,0));
        self.renderer.clear();
    }

    /*fn pressed_scancode_set(e: &sdl2::EventPump)
                            -> HashSet<Scancode> {
        e.keyboard_state().pressed_scancodes().collect()
    }

    /*fn pressed_keycode_set(e: &sdl2::EventPump) -> HashSet <Keycode> {
    e.keyboard_state().pressed_scancodes()
    .filter_map(Keycode::from_scancode())
    .collect()
}*/
    
    fn newly_pressed(old: &HashSet<Scancode>,
                     new: &HashSet<Scancode>)
                     -> HashSet<Scancode> {
        new - old
    }
 */   
}

fn lookup_keycode(key: u8) -> Keycode {
    match key {
        0  => Keycode::Num0,
        1  => Keycode::Num1,
        2  => Keycode::Num2,
        3  => Keycode::Num3,
        4  => Keycode::Num4,
        5  => Keycode::Num5,
        6  => Keycode::Num6,
        7  => Keycode::Num7,
        8  => Keycode::Num8,
        9  => Keycode::Num9,
        10 => Keycode::A,
        11 => Keycode::B,
        12 => Keycode::C,
        13 => Keycode::D,
        14 => Keycode::E,
        15 => Keycode::F,
        _ => Keycode::H
            
    }
}
