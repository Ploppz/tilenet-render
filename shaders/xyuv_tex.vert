#version 150

in vec2 pos;
/* in vec2 texpos; */

out vec2 f_texpos; // fragment texture position

/* uniform vec2 tex_lefttop; */
/* uniform vec2 tex_size; */

uniform vec2 tex_size;      // Size in pixels of texture
uniform vec2 view_size;     // viewport size
uniform vec2 screen_center; // in pixel coordinates


void main() {
    // `texpos` isn't really the texture position... it just helps us calculate it
    /* f_texpos = tex_lefttop + tex_size * texpos; */

    f_texpos = (screen_center + view_size/2.0 * pos) / tex_size;
    gl_Position = vec4(pos, 0, 1);
}
