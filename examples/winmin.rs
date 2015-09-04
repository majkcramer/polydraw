#![cfg(target_os = "windows")]

extern crate polydraw;
extern crate time;

mod support;

use polydraw::sys::win32;
use polydraw::sys::wgl;
use polydraw::sys::gl;

use support::{create_data, update_data};

fn main() {
   let width: u32 = 1280;
   let height: u32 = 720;

   let window = win32::Window::create(width, height, "PolyDraw", "WinMinClass");
   window.show_normal();

   let dc = window.dc();
   println!("DC: {:?}", dc);

   match wgl::init_pixel_format(dc) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   let context = match wgl::Context::create(dc) {
      Ok(context) => context,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("RC: {:?}", context.rc);

   gl::load(wgl::Loader::new());

   let mut counter: u64 = 0;
   let mut data = create_data(width, height);

   let texture = gl::Texture::new(width, height);

   println!("GL texture ................ : {:?}", texture.name);

   let framebuffer = gl::Framebuffer::new(&texture);

   println!("GL framebuffer ............ : {:?}", framebuffer.name);

   let start_time = time::precise_time_ns();

   let mut exit = false;
   let mut seed = 0;

   loop {
      loop {
         let message = match win32::Message::peek() {
            Some(message) => message,
            None => break
         };

         if message.is_quit() {
            exit = true;
         }

         message.translate();
         message.dispatch();
      }

      if exit {
         break;
      }

      counter += 1;
      seed = counter;

      update_data(&mut data, width, height, &mut seed);

      texture.update(width, height, &data);

      framebuffer.blit(width, height);

      gl::flush();

      wgl::swap_buffers(dc);
   }


   let end_time = time::precise_time_ns();

   println!("Time ns ................... : {:?}", end_time - start_time);
   println!("Cycles .................... : {:?}", counter);
   println!("FPS ....................... : {:?}", counter * 1000000000 / (end_time - start_time) );
}
