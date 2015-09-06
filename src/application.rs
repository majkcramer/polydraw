#[cfg(target_os = "windows")]
pub use os::windows::application::WindowsApplication as OsApplication;
#[cfg(target_os = "linux")]
pub use os::linux::application::LinuxApplication as OsApplication;

#[cfg(target_os = "windows")]
pub use os::windows::display::WindowsDisplay as OsDisplay;
#[cfg(target_os = "linux")]
pub use os::linux::display::LinuxDisplay as OsDisplay;

use super::frame::RenderFrame;
use super::renderer::Renderer;

pub struct Application {
   pub os_application: OsApplication,
   pub render_frame: RenderFrame,
}

use super::creator::ApplicationCreator;

impl Application {
   pub fn new<'a>() -> ApplicationCreator<'a> {
      let display = match OsDisplay::new() {
         Ok(os_application) => os_application,
         Err(e) => {
            panic!(e.description);
         }
      };

      ApplicationCreator::new(display)
   }

   pub fn create(
      display: OsDisplay,
      title: &str,
      x: u32, y: u32,
      width: u32, height: u32
   ) -> Self {
      let (screen_width, screen_height) = display.screen_size();

      let render_frame = RenderFrame::new(width, height, screen_width, screen_height);

      let os_application = match OsApplication::new(
         display, title, x, y, width, height
      ) {
         Ok(os_application) => os_application,
         Err(e) => {
            panic!(e.description);
         }
      };

      Application {
         os_application: os_application,
         render_frame: render_frame,
      }
   }

   pub fn run(&mut self, renderer: &mut Renderer) {
      match self.os_application.run(renderer, &mut self.render_frame) {
         Ok(_) => {},
         Err(e) => {
            panic!(e.description);
         }
      }
   }

   pub fn screen_size(&self) -> (u32, u32) {
      self.os_application.screen_size()
   }
}
