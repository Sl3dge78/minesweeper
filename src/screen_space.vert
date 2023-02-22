#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUv;
layout (location = 3) in vec4 aColor;

out vec2 TexCoords;
out vec4 VertexColor;

uniform mat4 Projection;

void main() {
    gl_Position = Projection * vec4(aPos.xy, 0.0, 1.0); 
    TexCoords = aUv;
    VertexColor = aColor;
}  
