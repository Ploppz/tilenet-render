#version 150

in vec2 f_texpos;
in vec2 f_next_texpos;
uniform sampler2D sampler;
uniform vec3 bg_col;

out vec4 out_color;


void main() {
    float inside = float(f_texpos.x >= 0.0 && f_texpos.y >= 0.0 && f_texpos.x <= 1.0 && f_texpos.y <= 1.0);
    // Supersample
    float intensity =
          texture(sampler, f_texpos).x
        + texture(sampler, vec2(f_texpos.x, f_next_texpos.y))
        + texture(sampler, vec2(f_next_texpos.x, f_next_texpos.y))
        + texture(sampler, vec2(f_next_texpos.x, f_texpos.y));

    vec3 tex_col = vec3(intensity);

    vec3 col = mix(bg_col, tex_col, inside);
    out_color = vec4(col, 1);
}
