#[derive(Debug)]
pub struct Font {
    font: *mut sdl3_ttf_sys::ttf::TTF_Font
}

#[derive(Debug)]
pub struct Surface {
    surface: *mut sdl3_sys::surface::SDL_Surface
}

impl Font {
    pub fn close(&self) {
        unsafe {
            sdl3_ttf_sys::ttf::TTF_CloseFont(self.font);
        }
    }

    pub fn render_shaded(
        &self,
        ch: u32,
        fg: sdl3_sys::pixels::SDL_Color
    ) -> Result<Surface, String> {
        unsafe {
            let surface = sdl3_ttf_sys::ttf::TTF_RenderGlyph_Solid(
                self.font,
                ch,
                fg
            );

            if surface.is_null() {
                Err(get_sdl_error())
            } else {
                Ok(Surface { surface })
            }
        }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        self.close();
    }
}

impl Surface {
    pub fn create(width: i32, height: i32) -> Result<Self, String> {
        let surface = unsafe {
            sdl3_sys::surface::SDL_CreateSurface(
                width,
                height,
                sdl3_sys::pixels::SDL_PIXELFORMAT_ARGB8888
            )
        };

        if surface.is_null() {
            Err(get_sdl_error())
        } else {
            Ok(Self { surface })
        }
    }

    pub fn width(&self) -> i32 {
        unsafe {
            (*self.surface).w
        }
    }

    pub fn height(&self) -> i32 {
        unsafe {
            (*self.surface).h
        }
    }

    pub fn destroy(&self) {
        unsafe {
            sdl3_sys::surface::SDL_DestroySurface(self.surface);
        }
    }

    pub fn blit(
        &self,
        src_rect: Option<sdl3_sys::rect::SDL_Rect>,
        dest: &mut Surface,
        dest_rect: Option<sdl3_sys::rect::SDL_Rect>
    ) -> Result<(), String> {
        unsafe {
            let src_rect = match src_rect {
                None => std::ptr::null(),
                Some(rect) => &rect
            };
            let dest_rect = match dest_rect {
                None => std::ptr::null(),
                Some(rect) => &rect
            };
            let result = sdl3_sys::surface::SDL_BlitSurface(
                self.surface,
                src_rect,
                dest.surface,
                dest_rect
            );

            if result {
                Ok(())
            } else {
                Err(get_sdl_error())
            }
        }
    }

    pub fn save_png(&self, path: &str) -> Result<(), String> {
        unsafe {
            let success =  sdl3_image_sys::image::IMG_SavePNG(
                self.surface,
                std::ffi::CString::new(path).unwrap().as_ptr()
            );

            if success {
                Ok(())
            } else {
                Err(get_sdl_error())
            }
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.destroy();
    }
}

pub fn init() -> Result<(), String> {
    unsafe {
        if sdl3_ttf_sys::ttf::TTF_Init() {
            Ok(())
        } else {
            Err(get_sdl_error())
        }
    }
}

pub fn quit() {
    unsafe {
        sdl3_ttf_sys::ttf::TTF_Quit();
    }
}

pub fn open_font(path: &str, size: f32) -> Result<Font, String> {
    let path = std::ffi::CString::new(path).unwrap();

    unsafe {
        let font = sdl3_ttf_sys::ttf::TTF_OpenFont(
            path.as_ptr(),
            size
        );

        if font.is_null() {
            Err(get_sdl_error())
        } else {
            Ok(Font { font })
        }
    }
}

fn get_sdl_error() -> String {
    unsafe {
        let error = sdl3_sys::error::SDL_GetError();
        let c_string = std::ffi::CStr::from_ptr(error);
        c_string.to_string_lossy().into_owned()
    }
}
