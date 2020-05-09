#version 400 core

in vec4 frag_color_vs;
flat in float frag_color_mod;

out vec4 frag_color;

void main() {
    frag_color = frag_color_vs * frag_color_mod;
}
