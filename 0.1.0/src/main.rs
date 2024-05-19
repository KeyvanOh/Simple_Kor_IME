use winit::window::Window;
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, ModifiersState};

use pixels::{Pixels, SurfaceTexture};
use pixels::wgpu::Color;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

mod keyboard;

use rustkorean::{classify_korean, KoreanType, check_korean};

fn main() {
	let mut event_loop = EventLoop::new();
	let window = Window::new(&event_loop).unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };
	let frame = pixels.frame_mut();
	for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
		let rgba = [255, 255, 0, 255];
		pixel.copy_from_slice(&rgba);
	};
	
	let mut is_typing = false;
	
	let mut is_korean = false;
	let mut is_capslock = false;

	let mut typed = "".to_owned();
	let mut typing: Vec<char> = [].to_vec();
	
	let mut test = 3;
	
	keyboard::change(&mut test);
	println!("{test}");
	
	event_loop.run(move |event, _, control_flow| {
		keyboard::change_typed(&mut typed, &mut typing, &event, &mut is_typing, &mut is_korean, &mut is_capslock, &mut pixels);
		
		control_flow.set_wait();		
	});
}