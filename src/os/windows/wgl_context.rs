use error::RuntimeError;

use sys::win32;
use sys::wgl;
use sys::gl;

pub struct WglContext {
   pub context: wgl::Context,
}

impl WglContext {
   pub fn new(device_context: &win32::DeviceContext) -> Result<Self, RuntimeError> {
      try!(wgl::init_pixel_format(device_context));

      let context = try!(wgl::Context::create(device_context));

      Self::init_gl();

      try!(wgl::swap_interval(0));

      Ok(WglContext {
         context: context,
      })
   }

   #[inline]
   pub fn init_gl() {
      let loader = wgl::Loader::new();
      gl::load(&loader);
      wgl::load(&loader);

      gl::reset_pixelstore_alignment();
      gl::enable_framebuffer_srgb();
   }
}
