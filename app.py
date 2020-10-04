from renderer import Renderer
from sandbox import Env, Agent
import random

import datetime as dt

start_time = dt.datetime.today().timestamp()
i = 0
if __name__ == "__main__":
    env = Env("polygons.json")
    renderer = Renderer(500)

    agent = Agent((0.5, 0.5), 0.0)
    direction_change = 0

    min_distance_to_obstacle = 0.01
    speed = 0.006
    slow_speed = 0.0001

    env_lines = env.lines

    agent.cast_rays()
    rays = agent.rays
    env_targets = env.targets
    while True:
        if min([r["length"] for r in rays]) < min_distance_to_obstacle:
            agent.step(random.random() / 2, slow_speed)
        else: 
            agent.step(direction_change, speed)
        env.update_agent(agent)
        rays = agent.rays
        renderer.draw(env_lines, rays, env_targets)

        time_diff = dt.datetime.today().timestamp() - start_time
        i += 1
        if i % 100 == 0: print(i / time_diff)

