#shader vertex
# version 300 es

layout (location=0) in vec2 pos;
layout (location=1) in vec4 color;
layout (location=2) in float z_index;

uniform mat4 model;
uniform mat4 projection;

out vec4 Col;

void main() {
    Col = color;
    gl_Position = projection * model * vec4(pos.x, pos.y, 0.0 + z_index, 1.0);
    gl_PointSize = 2.0;
}

#shader fragment
# version 300 es
precision mediump float;

uniform float alpha_value; 

in vec4 Col;
out vec4 FragColor;

void main() {
    FragColor = vec4(Col.rgb, alpha_value);
}