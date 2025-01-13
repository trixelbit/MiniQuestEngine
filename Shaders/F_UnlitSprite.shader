
#version 140

in vec2 v_tex_coords;
out vec4 color;

// engine information
uniform int time;


uniform float frame_count;
uniform float cell_x_count;
uniform float cell_y_count;
uniform float speed;

uniform sampler2D tex;

void main()
{
    int currentIndex = int(mod(time * speed, frame_count));

    vec2 cellSize
        = vec2(
            1.0 / cell_x_count,
            1.0 / cell_y_count
        );

    vec2 offset = vec2(
         mod(float(currentIndex), cell_x_count) / cell_x_count,
          1 - (0.5 * floor(2 * float(currentIndex) * cellSize.x * cellSize.y))
    );


    vec2 cellCoord = vec2(
        v_tex_coords.x * cellSize.x,
        -(1 - v_tex_coords.y) * cellSize.y
    );

    vec2 samplePoint = offset + cellCoord;
    color = texture(tex, samplePoint);

    if(color.a < .01)
    {
        discard;
    }
}
