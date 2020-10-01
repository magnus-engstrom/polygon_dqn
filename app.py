from renderer import Renderer
from sandbox import Env, Agent
import random

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
    while True:
        if min([r["length"] for r in rays]) < min_distance_to_obstacle:
            agent.step(random.random() / 2, slow_speed)
        else: 
            agent.step(direction_change, speed)
        env.update_agent(agent)
        rays = agent.rays
        renderer.draw(env_lines, rays)
