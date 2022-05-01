#version 330 core
out vec4 out_colour;

in vec2 v_tex;
in vec4 v_colour;
in vec2 v_sprite_pos;
in float v_tex_unit;

// uniform sampler2D textures[4];
uniform sampler2D texture_0;
uniform sampler2D texture_1;
uniform sampler2D texture_2;
uniform sampler2D texture_3;

// other sprites - hard coded for kennynl spritesheet atm
// const int kenny_num_cols = 48;
// const int kenny_num_rows = 22;
// const float kenny_scale_x = 1.0f / kenny_num_cols;
// const float kenny_scale_y = 1.0f / kenny_num_cols;
// const int spaceship_cols = 8;
// const int spaceship_rows = 8;
// const float spaceship_scale_x = 1.0f / spaceship_cols;
// const float spaceship_scale_y = 1.0f / spaceship_rows;
const int num_cols = 48;
const int num_rows = 22;
const float scale_x = 1.0f / num_cols;
const float scale_y = 1.0f / num_rows;

void
main()
{
  // int index = int(v_tex_unit);

  // out_colour = texture(textures[1], v_tex);
  out_colour = vec4(1.0f, 0.0f, 0.0f, 1.0f);
  return;

  // if (v_sprite_pos.x == 0 && v_sprite_pos.y == 0) { // a whole texture
  //   out_colour = v_colour;
  //   // out_colour += texture(textures[0], v_tex) * when_eq((1-sample_texture), 0);
  //   return;
  // } 

  // vec2 sprite_uv = vec2(
  //   v_tex.x / num_cols + v_sprite_pos.x * scale_x,
  //   v_tex.y / num_rows + v_sprite_pos.y * scale_y      
  // );

  // // out_colour = vec4(1.0) * texture(texture_0, sprite_uv);
  // // out_colour = texture(texture_1, v_tex);
  // out_colour = vec4(1.0f);
  return;

  // if(index == texture_id_kenny)
  // {
  //   sprite_uv = vec2(
  //     v_tex.x / kenny_num_cols + v_sprite_pos.x * kenny_scale_x,
  //     v_tex.y / kenny_num_rows + v_sprite_pos.y * kenny_scale_y      
  //   );
  // }
  // } else if(index == texture_id_spaceships)
  // {
  //   sprite_uv = vec2(
  //     v_tex.x / spaceship_cols + v_sprite_pos.x * spaceship_scale_x,
  //     v_tex.y / spaceship_rows + v_sprite_pos.y * spaceship_scale_y      
  //   );
  // }

  // out_colour = v_colour * texture(textures[0], sprite_uv);
  // out_colour = vec4(0.0f, 1.0f, 0.0f, 1.0f);
}

// if (do_lighting) {
//   // non-pixel, directly sample texture
//   vec2 uv = v_tex;
//   uv = uv_cstantos(uv, vec2(screen_w, screen_h));
//   vec4 tex_main = texture(textures[1], uv);
//   vec4 tex_shadow = texture(textures[2], uv);
//   vec4 r;
//   for (int i = 0; i < num_lights; i++) {
//     if (light_enabled[i]) {
//       float distance = length(light_pos[i] - FragPos);
//       const float light_constant = 1.0f;
//       float linear = light_linear[i];
//       float quadratic = light_quadratic[i];
//       float attenuation = 1.0 / (light_constant + linear * distance + quadratic * (distance * distance));
//       vec4 c = tex_main * attenuation;
//       r += c;
//     }
//   }
//   vec4 c = r;
//   // vec4 c = tex_main;
//   // pixel is in shadow
//   if (tex_shadow.r == 0.0f) {
//     c = c * vec4(0.92f, 0.92f, 0.92f, 1.0f);
//   }
//   out_colour = c;
//   out_colour.a = 1.0f;
//   return;
// }

// uniform bool do_lighting = false;
// const int num_lights = 32;
// uniform bool light_enabled[num_lights];
// uniform vec3 light_pos[num_lights];
// uniform float light_linear[num_lights];
// uniform float light_quadratic[num_lights];

// vec2
// uv_nearest(vec2 uv, ivec2 texture_size)
// {
//   vec2 pixel = uv * texture_size;
//   pixel = floor(pixel) + .5;
//   return pixel / texture_size;
// }

// vec2
// uv_cstantos(vec2 uv, vec2 res)
// {
//   vec2 pixels = uv * res;
//   // Updated to the final article
//   vec2 alpha = 0.7 * fwidth(pixels);
//   vec2 pixels_fract = fract(pixels);
//   vec2 pixels_diff = clamp(.5 / alpha * pixels_fract, 0, .5) + clamp(.5 / alpha * (pixels_fract - 1) + .5, 0, .5);
//   pixels = floor(pixels) + pixels_diff;
//   return pixels / res;
// }

// heartbeast approach?
// https://www.youtube.com/watch?v=2JbhkZe22bE&list=RDCMUCrHQNOyU1q6BFEfkNq2CYMA&index=25
// vec2 uv = v_tex;
// vec2 size = vec2(textureSize(textures[1], 0));
// vec2 pixel = vec2(1.0) / size;
// uv -= pixel * vec2(0.5);
// vec2 uv_pixels = uv * size;
// vec2 delta_pixel = fract(uv_pixels) - vec2(0.5);
// vec2 ddxy = fwidth(uv_pixels);
// vec2 mip = log2(ddxy) - 0.5;
// vec4 tex_main =
//   textureLod(textures[1], uv + (clamp(delta_pixel / ddxy, 0.0, 1.0) - delta_pixel) * pixel, min(mip.x, mip.y));

// fat pixel approach
// float tp = float(float(screen_h) / float(screen_w));
// vec2 tex_size = vec2(screen_w, screen_h);
// vec2 pixel = v_tex * tex_size;
// vec2 fat_pixel = floor(pixel) + 0.5;
// fat_pixel += 1 - clamp((1.0 - fract(pixel)) * tp, 0, 1); // subpixel aa algorithm
// vec2 uv = fat_pixel / tex_size;

// this approach?
// https://csantosbh.wordpress.com/2014/02/05/automatically-detecting-the-texture-filter-threshold-for-pixelated-magnifications/
// int w = int(screen_w);
// int h = int(screen_h);
// vec2 vUv = v_tex * vec2(w, h);
// vec2 alpha = 0.7 * vec2(dFdx(vUv.x), dFdy(vUv.y));
// vec2 x = fract(vUv);
// vec2 x_ = clamp(0.5 / alpha * x, 0.0, 0.5) + clamp(0.5 / alpha * (x - 1.0) + 0.5, 0.0, 0.5);
// vec2 uv = (floor(vUv) + x_) / vec2(w, h);