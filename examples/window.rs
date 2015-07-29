#![cfg(target_os = "linux")]

extern crate polydraw;

use polydraw::os::xcb;
use polydraw::os::x11;
use polydraw::os::egl;
use polydraw::os::gl;
use polydraw::os::cl;

fn print_screen_info(screen: &xcb::Screen) {
   println!("Informations of screen : {}", screen.root());
   println!("   width ............. : {}", screen.width_in_pixels());
   println!("   height ............ : {}", screen.height_in_pixels());
   println!("   white pixel ....... : {}", screen.white_pixel());
   println!("   black pixel ....... : {}", screen.black_pixel());
}

fn main() {
   let platforms = match cl::get_platforms() {
      Ok(platforms) => platforms,
      Err(e) => {
         panic!(e.description);
      }
   };

   for (i, platform) in platforms.iter().enumerate() {
      println!("CL platform [{}] ...... : {:?}", i, platform.ptr);
   }

   let display = match x11::Display::default() {
      Ok(display) => display,
      Err(e) => {
         panic!(e.description);
      }
   };

   let connection = match display.xcb_connection() {
      Ok(connection) => connection,
      Err(e) => {
         panic!(e.description);
      }
   };

   display.xcb_own_event_queue();

   let default_screen = display.default_screen();

   let scr = connection.screen_of_display(default_screen);

   print_screen_info(&scr);

   let window = connection.generate_id();

   println!("window ............... : {:?}", window);

   connection.create_window(
      window, &scr,
      0, 0, 800, 450,
   );

   connection.map_window(window);

   if !egl::bind_api(egl::API::OpenGL) {
      panic!("eglBindAPI failed");
   }

   let egl_d = egl::get_display(&display);
   let egl_display = egl_d.ptr;

   println!("egl display .......... : {:?}", egl_display);

   let version = match egl::initialize(&egl_d) {
      Ok(version) => version,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("egl version .......... : {:?}.{:?}", version.major, version.minor);

   let config = match egl::choose_config(&egl_d) {
      Ok(config) => config,
      Err(e) => {
         panic!(e.description);
      }
   };

   let context = match egl::create_context(&egl_d, &config) {
      Ok(context) => context,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("context ptr .......... : {:?}", context.ptr);

   let surface = match egl::create_window_surface(&egl_d, &config, &window) {
      Ok(surface) => surface,
      Err(e) => {
         panic!(e.description);
      }
   };

   println!("surface ptr .......... : {:?}", surface.ptr);

   match egl::make_current(&egl_d, &surface, &surface, &context) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   match egl::query_context(&egl_d, &context) {
      Ok(_) => {},
      Err(e) => {
         panic!(e.description);
      }
   };

   loop {
      let event = match connection.wait_for_event() {
         None => {
            return;
         },
         Some(event) => event
      };

      let event_type = event.event_type();

      match event_type {
         xcb::EventType::KeyPress => {
            break;
         },
         xcb::EventType::Expose => {
            gl::clear_color(0.0, 0.7, 1.0, 1.0);
            gl::clear();
            gl::flush();

            match egl::swap_buffers(&egl_d, &surface) {
               Ok(_) => {},
               Err(e) => {
                  panic!(e.description);
               }
            };
         }
         _ => {}
      }
   }

   connection.destroy_window(window);
}
