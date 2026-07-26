use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;
const SCALE: u32 = 3;

pub struct Display {
    pub canvas: Canvas<Window>,
    pub sdl: Sdl,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Display {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video
            .window("NES Emulator", WIDTH * SCALE, HEIGHT * SCALE)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let texture_creator = canvas.texture_creator();

        Self {
            canvas,
            sdl,
            texture_creator,
        }
    }
    pub fn create_frame_texture(texture_creator: &TextureCreator<WindowContext>) -> Texture {
        texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, WIDTH, HEIGHT)
            .unwrap()
    }

    pub fn render_frame(canvas: &mut Canvas<Window>, texture: &mut Texture, frame_buffer: &[u32]) {
        texture
            .with_lock(None, |buf: &mut [u8], pitch: usize| {
                for y in 0..HEIGHT as usize {
                    for x in 0..WIDTH as usize {
                        let pixel = frame_buffer[y * WIDTH as usize + x];
                        let offset = y * pitch + x * 4;

                        let r = ((pixel >> 16) & 0xFF) as u8;
                        let g = ((pixel >> 8) & 0xFF) as u8;
                        let b = (pixel & 0xFF) as u8;

                        buf[offset] = b;
                        buf[offset + 1] = g;
                        buf[offset + 2] = r;
                        buf[offset + 3] = 0xFF;
                    }
                }
            })
            .unwrap();

        canvas.clear();
        canvas.copy(texture, None, None).unwrap();
        canvas.present();
    }
}
