#version 150

in vec2 texpos;

uniform vec2 texsize;      // number of texels in texture
uniform sampler2D sampler;
uniform vec3 bg_col;

out vec4 out_color;


void main() {
    vec2 step = vec2(dFdx(texpos.x), dFdy(texpos.y));

    vec2 t = clamp( (0.5 - fract(texpos))/step, 0, 1);


    // Supersample texel neighbors -- bilinear interpolation, based on t

    float q00 = texture(sampler, floor(texpos            )/texsize).r;
    float q01 = texture(sampler, floor(texpos + vec2(0,1))/texsize).r;
    float q11 = texture(sampler, floor(texpos + vec2(1,1))/texsize).r;
    float q10 = texture(sampler, floor(texpos + vec2(1,0))/texsize).r;

    float q0y = mix(q00, q01, t.y);
    float q1y = mix(q10, q11, t.y);
    float qxy = mix(q0y, q1y, t.x);

    float inside = float(texpos.x >= 0.0 && texpos.y >= 0.0 && texpos.x <= texsize.x && texpos.y <= texsize.y);
    vec3 col = mix(bg_col, vec3(qxy), inside);
    out_color = vec4(col, 1);
}
