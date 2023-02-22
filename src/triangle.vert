#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUv;
layout (location = 3) in vec3 aColor;

uniform mat4 Projection;
uniform mat4 View;
uniform mat4 Model;

out vec3 Normal;
out vec2 UV;
out vec3 FragPos;
out vec3 VertexColor;

void main() {
    Normal = aNormal;
    UV = aUv;
    VertexColor = aColor;
    FragPos = vec3(Model * vec4(aPos, 1.0));
    gl_Position = Projection * View * vec4(FragPos, 1.0);
}
