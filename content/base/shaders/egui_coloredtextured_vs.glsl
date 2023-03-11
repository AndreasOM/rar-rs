#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 texCoords;
layout (location = 2) in vec4 color;
out vec2 oTexCoords;
out vec4 oColor;

uniform mat4 modelViewProjectionMatrixReal;

void main()
{
	oTexCoords = texCoords;
	oColor = color;
	// oColor = color * vec4( 1.0, 0.5, 0.5, 0.5 );
    gl_Position = modelViewProjectionMatrixReal * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
