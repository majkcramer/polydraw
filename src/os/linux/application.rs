use std::rc::Rc;

use error::RuntimeError;

use sys::x11;
use sys::xcb;
use sys::egl;

use super::window::LinuxWindow;

pub struct LinuxApplication {
   pub x11_display: X11DisplayHandler,
   pub connection: ConnectionHandler,
   pub screen: ScreenHandler,
   pub egl_display: EglDisplayHandler,
}

impl LinuxApplication {
   pub fn new() -> Result<Self, RuntimeError> {
      let x11_display = try!(X11DisplayHandler::new());
      let connection = try!(ConnectionHandler::new(&x11_display));
      let screen = try!(ScreenHandler::new(&x11_display, &connection));
      let egl_display = try!(EglDisplayHandler::new(&x11_display));

      Ok(LinuxApplication {
         x11_display: x11_display,
         connection: connection,
         screen: screen,
         egl_display: egl_display,
      })
   }

   #[inline]
   pub fn screen_size(&self) -> (u32, u32) {
      self.screen.size()
   }

   pub fn create_os_window(
      &self, title: &str, x: u32, y: u32, width: u32, height: u32
   ) -> Result<LinuxWindow, RuntimeError> {
      let xcb_window = try!(self.screen.create_window(
         &self.connection, x, y, width, height
      ));

      Ok(LinuxWindow::new(xcb_window, title))
   }
}

pub struct X11DisplayHandler {
   pub display: x11::Display,
}

impl X11DisplayHandler {
   #[inline]
   pub fn new() -> Result<Self, RuntimeError> {
      let display = try!(x11::Display::default());

      display.xcb_own_event_queue();

      Ok(X11DisplayHandler {
         display: display
      })
   }

   #[inline]
   pub fn connection(&self) -> Result<xcb::Connection, RuntimeError> {
      self.display.xcb_connection()
   }

   #[inline]
   pub fn screen_id(&self) -> x11::ScreenID {
      self.display.default_screen()
   }
}

pub struct ConnectionHandler {
   pub connection: Rc<xcb::Connection>,
}

impl ConnectionHandler {
   #[inline]
   pub fn new(display: &X11DisplayHandler) -> Result<Self, RuntimeError> {
      Ok(ConnectionHandler {
         connection: Rc::new(try!(display.connection()))
      })
   }

   #[inline]
   pub fn screen_of_display(&self, display: &X11DisplayHandler) -> Result<xcb::Screen, RuntimeError> {
      let screen_id = display.screen_id();

      self.connection.screen_of_display(&screen_id)
   }
}

pub struct ScreenHandler {
   pub screen: xcb::Screen,
}

impl ScreenHandler {
   #[inline]
   pub fn new(display: &X11DisplayHandler, connection: &ConnectionHandler) -> Result<Self, RuntimeError> {
      Ok(ScreenHandler {
         screen: try!(connection.screen_of_display(display))
      })
   }

   #[inline]
   pub fn size(&self) -> (u32, u32) {
      (
         self.screen.width_in_pixels() as u32,
         self.screen.height_in_pixels() as u32
      )
   }

   #[inline]
   pub fn create_window(
      &self, connection: &ConnectionHandler, x: u32, y: u32, width: u32, height: u32
   ) -> Result<xcb::Window, RuntimeError> {

      xcb::Window::create(
         &connection.connection, &self.screen, x, y, width, height,
      )
   }
}

pub struct EglDisplayHandler {
   pub display: egl::Display,
   pub version: egl::Version,
   pub config: egl::Config,
   pub context: egl::Context,
}

impl EglDisplayHandler {
   pub fn new(x11_display: &X11DisplayHandler) -> Result<Self, RuntimeError> {
      try!(egl::bind_api(egl::API::OpenGL));

      let display = try!(egl::Display::from_native(&x11_display.display));
      let version = try!(egl::initialize(&display));
      let config = try!(egl::choose_config(&display));
      let context = try!(egl::create_context(&display, &config));

      try!(egl::query_context(&display, &context));

      Ok(EglDisplayHandler {
         display: display,
         version: version,
         config: config,
         context: context,
      })
   }

   #[inline]
   pub fn create_surface(&self, window: &LinuxWindow) -> Result<egl::Surface, RuntimeError> {
      egl::create_window_surface(
         &self.display,
         &self.config,
         &window.window.window_id.id
      )
   }
}

pub struct EglSurfaceHandler {
   pub surface: egl::Surface,
}

impl EglSurfaceHandler {
   #[inline]
   pub fn new(display: &EglDisplayHandler, window: &LinuxWindow) -> Result<Self, RuntimeError> {
      let surface = try!(display.create_surface(&window));

      Ok(EglSurfaceHandler {
         surface: surface,
      })
   }
}
