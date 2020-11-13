pub mod render_gl;

use std::ffi::CStr;
use std::ffi::CString;

fn main() {
	let size: glutin::dpi::LogicalSize<u32> = (800, 600).into();

	let el = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new()
		.with_title("Hello rust gl!")
		.with_inner_size(size)
		.with_resizable(true);


	let windowed_context =
		glutin::ContextBuilder::new()
		.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 5)))
		.with_gl_profile(glutin::GlProfile::Core)
		.with_gl_debug_flag(true)
		.build_windowed(wb, &el)
		.unwrap();

	let windowed_context = unsafe { windowed_context.make_current().unwrap() };

	println!(
		"Pixel format of the window's GL context: {:?}",
		windowed_context.get_pixel_format()
	);

	let _gl = gl::load_with(|s| windowed_context.get_proc_address(s) as *const std::os::raw::c_void);

	let version = unsafe {
		let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
			.to_bytes()
			.to_vec();
		String::from_utf8(data).unwrap()
	};

	println!("OpenGL version {}", version);

	let vert_shader =
		render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
		.unwrap();

	let frag_shader =
		render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
			.unwrap();

	let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

	shader_program.set_used();



	let vertices: Vec<f32> = vec![
		// positions	  // colors
		0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
		-0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
		0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
	];

	let mut vbo: gl::types::GLuint = 0;
	unsafe {
		gl::GenBuffers(1, &mut vbo);
	}

	unsafe {
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,													   // target
			(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
			vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
			gl::STATIC_DRAW,							   // usage
		);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}


	// set up vertex array object

	let mut vao: gl::types::GLuint = 0;
	unsafe
	{
		gl::GenVertexArrays(1, &mut vao);
	}

	unsafe {
		gl::BindVertexArray(vao);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

		gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
		gl::VertexAttribPointer(
			0,		 // index of the generic vertex attribute ("layout (location = 0)")
			3,		 // the number of components per generic vertex attribute
			gl::FLOAT, // data type
			gl::FALSE, // normalized (int-to-float conversion)
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
			std::ptr::null(),									 // offset of the first component
		);
		gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
		gl::VertexAttribPointer(
			1,		 // index of the generic vertex attribute ("layout (location = 0)")
			3,		 // the number of components per generic vertex attribute
			gl::FLOAT, // data type
			gl::FALSE, // normalized (int-to-float conversion)
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
			(3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
		);

		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);
	}
	// set up shared state for window

	unsafe {
		gl::Viewport(0, 0, size.width as i32, size.height as i32);
		gl::ClearColor(0.3, 0.3, 0.5, 1.0);
	}


	println!("Size: {}:{}", size.width, size.height);

	el.run(move |event, _, control_flow|
	{
		//println!("{:?}", event);
		*control_flow = glutin::event_loop::ControlFlow::Wait;

		match event {
			glutin::event::Event::LoopDestroyed => return,
			glutin::event::Event::WindowEvent { event, .. } => match event
			{
				glutin::event::WindowEvent::Resized(physical_size) =>
				{
					unsafe
					{
						gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
						gl::ClearColor(0.3, 0.3, 0.5, 1.0);
					}
					windowed_context.resize(physical_size)

				}
				glutin::event::WindowEvent::CloseRequested =>
				{
					*control_flow = glutin::event_loop::ControlFlow::Exit
				}
				glutin::event::WindowEvent::KeyboardInput
				{
					input: glutin::event::KeyboardInput
					{
							virtual_keycode: Some(virtual_code),
							state,
							..
					},
					..
				} => match (virtual_code, state)
				{
					(glutin::event::VirtualKeyCode::Escape, _) =>
					{
						*control_flow = glutin::event_loop::ControlFlow::Exit
					}
					(glutin::event::VirtualKeyCode::W, glutin::event::ElementState::Pressed) =>
					{
					}
					(glutin::event::VirtualKeyCode::A, glutin::event::ElementState::Pressed) =>
					{
					}
					(glutin::event::VirtualKeyCode::S, glutin::event::ElementState::Pressed) =>
					{
					}
					(glutin::event::VirtualKeyCode::D, glutin::event::ElementState::Pressed) =>
					{
					}
					_ => (),
				},
				_ => (),
			},
			glutin::event::Event::RedrawRequested(_) =>
			{
				shader_program.set_used();
				unsafe
				{
					gl::BindVertexArray(vao);
					gl::DrawArrays(
						gl::TRIANGLES, // mode
						0,			 // starting index in the enabled arrays
						3,			 // number of indices to be rendered
					);
				}
				//gl.draw_frame([0.0, 0.5, 0.7, 1.0]);
				windowed_context.swap_buffers().unwrap();
			}
			_ => (),
		}
	});
}