use std::ptr;

use draw::RGB;
use sys::gl::{Texture, Framebuffer, Buffer};
use renderer::Renderer;

pub struct Frame {
   pub width: u32,
   pub height: u32,
   gl_context: FrameGLContext,
}

impl Frame {
   #[inline]
   pub fn new(width: u32, height: u32) -> Self {
      Frame {
         width: width,
         height: height,
         gl_context: FrameGLContext::new(width, height)
      }
   }

   #[inline]
   pub fn clear(&mut self) {
      self.gl_context.clear();
   }

   #[inline]
   pub fn put_pixel(&mut self, x: i32, y: i32, color: &RGB) {
      self.gl_context.put_pixel(x, y, color, self.width, self.height);
   }

   #[inline]
   pub fn resize(&mut self, width: u32, height: u32) {
      self.width = width;
      self.height = height;

      self.gl_context.resize(width, height);
   }

   #[inline]
   pub fn render(&mut self, renderer: &mut Renderer) {
      self.gl_context.pre_render();

      renderer.render(self);

      self.gl_context.post_render(self.width, self.height);
   }
}

struct FrameGLContext {
   pub texture: Texture,
   pub framebuffer: Framebuffer,
   pub buffer: Buffer,
}

impl FrameGLContext {
   #[inline]
   pub fn new(width: u32, height: u32) -> Self {
      let texture = Texture::new(width, height);
      let framebuffer = Framebuffer::new(&texture);
      let mut buffer = Buffer::new();

      texture.bind();
      framebuffer.bind();

      buffer.bind();
      buffer.init_data((width * height * 4) as usize);

      FrameGLContext {
         texture: texture,
         framebuffer: framebuffer,
         buffer: buffer,
      }
   }

   #[inline]
   pub fn clear(&mut self) {
      unsafe {
         ptr::write_bytes(
            self.buffer.ptr as *mut u8, 0, self.buffer.size as usize
         )
      };
   }

   #[inline]
   pub fn put_pixel(&mut self, x: i32, y: i32, color: &RGB, width: u32, height: u32) {
      if x >= width as i32 || y >= height as i32 || x < 0 || y < 0 {
         return;
      }

      let i = 4 * (x + y * width as i32) as isize;
      let p = self.buffer.ptr as *mut u8;
      unsafe {
         *p.offset(i) = color.r;
         *p.offset(i+1) = color.g;
         *p.offset(i+2) = color.b;
      }
   }

   #[inline]
   pub fn resize(&mut self, width: u32, height: u32) {
      self.buffer.init_data((width * height * 4) as usize);
      self.texture.resize(width, height);
   }

   #[inline]
   pub fn pre_render(&mut self) {
      self.buffer.map();
   }

   #[inline]
   pub fn post_render(&mut self, width: u32, height: u32) {
      self.buffer.unmap();

      self.texture.update(width, height);

      self.framebuffer.blit(width, height);
   }
}
