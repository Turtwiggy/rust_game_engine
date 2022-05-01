// your header
#include "cycle_sprite.hpp"

// components
#include "gameplay_components/components.hpp"
#include "modules/renderer/components.hpp"

void
game2d::update_cycle_sprite_system(entt::registry& registry, engine::Application& app)
{
  const auto& ri = registry.ctx<SINGLETON_RendererInfo>();

  // {
  //   const auto& view = registry.view<PlayerComponent>();
  //   view.each([](const auto& player) {
  //     //
  //   });
  // }
};
