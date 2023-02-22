
#version 330 core
out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;
in vec2 UV;
in vec3 VertexColor;

float constant = 1.0f;
float linear = 0.09f;
float quadratic = 0.002f;

void main() {
    vec3 light_pos = vec3(0, 2, 0);
    float distance = length(light_pos - FragPos);
    float attenuation = 1.0 / (constant + linear * distance + quadratic * (distance * distance));

    FragColor = attenuation * vec4(VertexColor, 1.0f);
} 
