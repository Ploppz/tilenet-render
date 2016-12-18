#version 150

in vec2 pos;

out vec2 screenpos;
out vec2 texpos;        // fragment texture position
out vec2 next_texpos;   // with x+1,y+1 in pixel coords
out vec2 step;          // Step in texture space to move one pixel in screen space

uniform vec2 tex_size;      // Size in pixels of texture
uniform vec2 view_size;     // viewport size
uniform vec2 screen_center; // in pixel coordinates


void main() {
    step = view_size / tex_size;

    screenpos = (pos+1.0)/2.0 * view_size;

    texpos = (screen_center + view_size/2.0 * pos) / tex_size;
    next_texpos = (screen_center + view_size/2.0 * pos + step) / tex_size;

    gl_Position = vec4(pos, 0, 1);
}
