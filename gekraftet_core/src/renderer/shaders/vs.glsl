#version 400 core

uniform float time;
uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 uv;

out vec4 frag_color_vs;
out float frag_color_mod;

void main() {
    frag_color_vs = color;
    frag_color_mod = uv.x;

    gl_Position =  projection * view * model * vec4(pos, 1.0);
}
