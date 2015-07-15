#[macro_use]
extern crate polydraw;

use polydraw::platform::x11::ffi;
use polydraw::os::xcb;
use polydraw::os::x11;
use polydraw::os::egl;
use polydraw::os::gl;

fn print_screen_info(screen: &xcb::Screen) {
   println!("Informations of screen : {}", screen.root());
   println!("   width ............. : {}", screen.width_in_pixels());
   println!("   height ............ : {}", screen.height_in_pixels());
   println!("   white pixel ....... : {}", screen.white_pixel());
   println!("   black pixel ....... : {}", screen.black_pixel());
}

fn main() {
   let display = match x11::Display::default() {
      Ok(display) => display,
      Err(e) => {
         panic!(e.description);
      }
   };

   let conn = match display.xcb_connection() {
      Ok(conn) => conn,
      Err(e) => {
         panic!(e.description);
      }
   };

   let connection = conn.ptr as *mut ffi::xcb_connection_t;

   display.xcb_own_event_queue();

   let default_screen = display.default_screen();

   let scr = conn.screen_of_display(default_screen);

   print_screen_info(&scr);

   let window = conn.generate_id();

   println!("window ............... : {:?}", window);

   conn.create_window(
      window, &scr,
      0, 0, 800, 450,
   );

   conn.map_window(window);

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
      let event = unsafe {
         ffi::xcb_wait_for_event(connection)
      };
      if event.is_null() {
         break;
      }

      let event_type = unsafe { (*event).response_type & !0x80 };

      match event_type {
         ffi::XCB_KEY_PRESS => {
            unsafe { ffi::free(event as *mut ffi::c_void) };
            break;
         },
         ffi::XCB_EXPOSE => {
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

      unsafe { ffi::free(event as *mut ffi::c_void) };
   }

   unsafe {
      ffi::xcb_destroy_window(connection, window)
   };
}
