#version 330 core
out vec4 FragColor;
  
in vec2 TexCoords;
in vec4 VertexColor;

uniform sampler2D texture0;

void main() { 
    vec4 tex = texture(texture0, TexCoords);
    FragColor = tex;
    // FragColor = vec4(VertexColor.xyz, tex.a * VertexColor.a);
}
