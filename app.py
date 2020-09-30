from renderer import Renderer
from sandbox import Env, Agent
import random

if __name__ == "__main__":
    env = Env("polygons.json")
    renderer = Renderer(500)

    agent = Agent((0.5, 0.5), 0.0)
    direction_change = 0

    min_distance_to_obstacle = 0.005
    speed = 0.0004
    slow_speed = 0.00005

    env_line_strings = env.line_strings

    agent.cast_rays()
    #rays return [[(start_x,start_y,end_x,end_y,length)]] ie rays[line_strings[(start_x,start_y,end_x,end_y,length), ..], ..]
    rays_line_stings = agent.rays
    while True:
        if min([r[0][4] for r in rays_line_stings]) < min_distance_to_obstacle:
            agent.step(random.random() / 2, slow_speed)
        else: 
            agent.step(direction_change, speed)
        env.update_agent(agent)
        rays_line_stings = agent.rays
        #print(rays)
        renderer.draw(env_line_strings, rays_line_stings)
