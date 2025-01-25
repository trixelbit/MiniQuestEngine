
#version 140

in vec2 v_tex_coords;
in vec3 v_normal;
out vec4 color;

// engine information
uniform int time;


// sprite information
uniform bool is_lit;
uniform int current_index;
uniform float pixel_dimension_x;
uniform float pixel_dimension_y;
uniform float frame_count;
uniform float cell_x_count;
uniform float cell_y_count;
uniform float speed;

uniform sampler2D tex;

void main()
{
  

  // SPRITE SHEET ANIMATION
  //int currentIndex = int(mod(time * speed, frame_count));

  vec2 cellSize
      = vec2(
          1.0 / cell_x_count,
          1.0 / cell_y_count
      );


  float modValue = mod(float(current_index), cell_x_count);
  float uv_x = modValue / cell_x_count;

  float y_cell = cell_y_count - floor((current_index) / cell_x_count);
  float uv_y = y_cell / cell_y_count;


  vec2 offset = vec2(uv_x, uv_y);


  vec2 cellCoord = vec2(
      v_tex_coords.x * cellSize.x,
      -(1.0 - v_tex_coords.y) * cellSize.y
  );


  // FINAL OUTPUT
  vec2 samplePoint = offset + cellCoord;
  color = texture(tex, samplePoint);



  float pixel_size_x = 1.0 / float(pixel_dimension_x);
  float pixel_size_y = 1.0 / float(pixel_dimension_y);
  vec2 pixel_size = vec2(pixel_size_x, pixel_size_y);


  float transparent = 0.01;
  if(color.a < transparent)
  {
    discard;
  }

/*  SPRITE OUTLINE SHADER



//  if(color.a < transparent)
//  {
//    if( 
//        (
//        texture(tex, samplePoint + (vec2(1,0) * pixel_size)).a > transparent ||
//        texture(tex, samplePoint + (vec2(-1,0) * pixel_size)).a > transparent ||
//        texture(tex, samplePoint + (vec2(0,1) * pixel_size)).a > transparent ||
//        texture(tex, samplePoint + (vec2(0,-1) * pixel_size)).a > transparent 
//        )
//
//    )
//    {
//      color = vec4(0,0,0,0);
//    
//    }
//    else 
//    {
//        discard;
//    }
//
//  }
*/

  if(!is_lit)
  {
    return;
  }

  // LIGHTING

  // blue
  vec4 shadow_color = vec4(0.02, 0.02, 0.1, 1);
  vec4 light_color = vec4(0.6, 0.9, 1.0, 0.5);

  // yellow green
  //vec4 shadow_color = vec4(0.1, 0.2, 0.2, 1);
  //vec4 light_color = vec4(1.0, 0.9, 0.3, 0.5);

  // white
  //vec4 shadow_color = vec4(0.14, 0.14, 0.14, 1);
  //vec4 light_color = vec4(1.0, 1.0, 1.0, 1.0);


  float intensity = 1;
  
  vec3 direction = normalize(vec3(
    0,-1, -0.5
    ));
    
    // demo settings
    //sin( float(time) / 800.0), 
    //cos(float(time) / 800.0), 
    //1 * sin(time / 1200.0) - 1

    //1,
    //-2, 
    //2 * sin(time / 1000.0) - 1//-2

    //sin( float(time) / 500.0), 
    //cos(float(time) / 500.0), 
    //-2 * sin(float(time) / 1000.0)
    //));

  vec4 shadowAmount = dot(v_normal, -direction) * vec4(1,1,1,1);
  vec4 shadowMix = mix(color * light_color, shadow_color, shadowAmount.x);

  color = shadowMix;

  // rim lighting
  vec2 light_offset = pixel_size * -normalize(vec2(direction.x, direction.y));
  vec4 value = texture(tex, samplePoint + (light_offset));
  vec4 value2 = texture(tex, samplePoint + (light_offset * 2));

  //vec4 mixedHighlight = mix(color, )

  if(value.a < 0.01)
  {
    color = color + (light_color * shadowAmount);
  }
  else if(value2.a < 0.01)
  {
    color = color + (light_color / 1.2 * shadowAmount);
  }

  color = max(color, vec4(0,0,0,0));

}










