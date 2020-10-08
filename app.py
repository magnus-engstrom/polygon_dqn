from renderer import Renderer
from sandbox import Env
import random

import datetime as dt

start_time = dt.datetime.today().timestamp()
i = 0
if __name__ == "__main__":
    env = Env("polygons.json")
    renderer = Renderer(500)

    direction_change = 0

    min_distance_to_obstacle = 0.01
    speed = 0.006
    slow_speed = 0.0001

    env_lines = env.lines

    rays = [999]
    env_targets = env.targets
    while True:
        if min(rays) < min_distance_to_obstacle:
            (state, reward, end) = env.step(random.randint(0, 4))
        else:
            (state, reward, end) = env.step(2)

        if end:
            env.reset()
            rays = [999]
            continue

        target_distance, target_bearing, *rays = state

        renderer.draw(env_lines, env.get_agent_rays(), env_targets)

        time_diff = dt.datetime.today().timestamp() - start_time
        i += 1
        if i % 100 == 0: print(i / time_diff)

