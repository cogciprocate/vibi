
// Vertex Shader:
#[allow(non_upper_case_globals)]
static vertex_shader_src: &'static str = stringify!(
    #version 330

    in vec2 position;
    in vec3 normal;
	in vec3 color;

	out vec3 Color;

	uniform mat4 model;
	uniform mat4 view;
	uniform mat4 proj;

	void main() {
		Color = color;

		uint grid_dim = 256 << 2;

		float border = 0.01;

		float x_scl = 0.086602540378f + border;
		float y_scl = 0.05 + border;

		float u = float(uint(gl_InstanceID) % grid_dim);
		float v = float(uint(gl_InstanceID) / grid_dim);

	 	float x_pos = ((v + u) * x_scl) + position.x;
	 	float y_pos = ((v * -y_scl) + (u * y_scl)) + position.y;

		gl_Position = proj * view * model * vec4(x_pos, y_pos, 0.0, 1.0);
	};
);


// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = stringify!(
	in vec3 Color;
	out vec4 outColor;
	void main() {
		outColor = vec4(Color, 1.0);
	};
)
