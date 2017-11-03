use caper::shader::{ default, texture };
use caper::game::Game;

pub fn add_custom_shaders(game: &mut Game) {
    let shaders = &mut game.renderer.shaders;
    let display = &game.renderer.display;

    let _ = shaders.add_shader(
        display,
        "contours",
        default::gl330::VERT,
        texture::gl330::FRAG,
        contours::GEOM,
        contours::TESS_CONTROL,
        contours::TESS_EVAL,
    );
}


mod contours {
    /// tessellation control shader
    pub const TESS_CONTROL: &'static str = "
        #version 400

        layout(vertices = 3) out;

        in vec3 v_normal[];
        in vec2 v_texture[];

        out vec3 tc_normal[];
        out vec2 tc_texture[];

        const float outer = 1.0;
        const float inner = 3.0;

        void main() {
            tc_normal[gl_InvocationID] = v_normal[gl_InvocationID];
            tc_texture[gl_InvocationID] = v_texture[gl_InvocationID];
            gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

            float d_01 = distance(gl_out[0].gl_Position, gl_out[1].gl_Position);
            float d_02 = distance(gl_out[0].gl_Position, gl_out[2].gl_Position);
            float d_12 = distance(gl_out[1].gl_Position, gl_out[2].gl_Position);

            float inner_t = max(d_01, max(d_02, d_12)) / 0.22;

            gl_TessLevelOuter[0] = outer;
            gl_TessLevelOuter[1] = outer;
            gl_TessLevelOuter[2] = outer;
            gl_TessLevelInner[0] = inner_t;
        }
    ";
    /// Default tessellation evaluation shader
    pub const TESS_EVAL: &'static str = "
        #version 400

        uniform mat4 projection_matrix;
        uniform mat4 modelview_matrix;

        layout(triangles, equal_spacing, ccw) in;

        in vec3 tc_normal[];
        in vec2 tc_texture[];

        out vec3 te_normal;
        out vec3 te_pos;
        out vec2 te_texture;

        vec3 tess_calc (vec3 one, vec3 two, vec3 three) {
            return ((gl_TessCoord.x) * one) +
                            ((gl_TessCoord.y) * two) +
                            ((gl_TessCoord.z) * three);
        }

        vec2 tex_calc (vec2 one, vec2 two, vec2 three) {
            return ((gl_TessCoord.x) * one) +
                            ((gl_TessCoord.y) * two) +
                            ((gl_TessCoord.z) * three);
        }

        void main () {
            te_normal = tess_calc(tc_normal[0], tc_normal[1], tc_normal[2]);

            vec3 position = tess_calc(gl_in[0].gl_Position.xyz,
                gl_in[1].gl_Position.xyz,
                gl_in[2].gl_Position.xyz);

            te_pos = position;

            vec2 texture = tex_calc(tc_texture[0], tc_texture[1], tc_texture[2]);
            te_texture = texture;

            gl_Position = projection_matrix *
                modelview_matrix *
                vec4(position, 1.0);
        }
    ";
    /// Line geometry shader for wireframes
    pub const GEOM: &'static str = "
        #version 330

        layout(triangles) in;
        layout(line_strip, max_vertices=3) out;

        in vec3 te_normal[];
        in vec3 te_pos[];
        in vec2 te_texture[];

        out vec3 g_normal;
        out vec3 g_pos;
        out vec2 g_texture;

        void main(void) {
            for(int i = 0; i < gl_in.length(); i++){
                g_normal = te_normal[i];
                g_pos = te_pos[i];
                gl_Position = gl_in[i].gl_Position;
                EmitVertex();
            }
            EndPrimitive();
        }
    ";
}
