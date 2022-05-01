#pragma once

// engine headers
#include "engine/application.hpp"

// other lib headers
#include <entt/entt.hpp>

namespace game2d {

void
update_cycle_sprite_system(entt::registry& registry, engine::Application& app);

} // namespace game2d