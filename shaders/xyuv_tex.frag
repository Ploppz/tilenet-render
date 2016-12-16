#version 150

in vec2 f_texpos;
uniform sampler2D sampler;
uniform vec3 bg_col;

out vec4 out_color;


void main() {
    float inside = float(f_texpos.x >= 0.0 && f_texpos.y >= 0.0 && f_texpos.x <= 1.0 && f_texpos.y <= 1.0);
    vec3 tex_col = vec3(texture(sampler, f_texpos).x);
    vec3 col = mix(bg_col, tex_col, inside);
    out_color = vec4(col, 1);
}
