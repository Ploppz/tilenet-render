#version 150

in vec2 screenpos;
in vec2 texpos;
in vec2 next_texpos;

uniform sampler2D sampler;
uniform vec3 bg_col;

out vec4 out_color;


void main() {
    float inside = float(texpos.x >= 0.0 && texpos.y >= 0.0 && texpos.x <= 1.0 && texpos.y <= 1.0);
    vec2 subpixel = fract(screenpos);
    // Supersample -- bilinear interpolation pased on subpixel value
    float q00 = texture(sampler, texpos).x;
    float q01 = texture(sampler, vec2(texpos.x, next_texpos.y)).x;
    float q11 = texture(sampler, vec2(next_texpos.x, next_texpos.y)).x;
    float q10 = texture(sampler, vec2(next_texpos.x, texpos.y)).x;

    float q0y = mix(q00, q01, subpixel.y);
    float q1y = mix(q10, q11, subpixel.y);
    float qxy = mix(q0y, q1y, subpixel.x);


    vec3 col = mix(bg_col, vec3(qxy), inside);
    out_color = vec4(col, 1);
}
