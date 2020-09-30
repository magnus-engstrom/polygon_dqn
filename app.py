import pygame
from renderer import Renderer
from environment import Environment
from agent import Agent
import random

if __name__ == "__main__":
    env = Environment("polygons.json", 500)
    renderer = Renderer(500)

    agent = Agent((250, 250))
    direction_change = 0

    while True:
        if min([r.length for r in agent.rays]) < 5:
            agent.move(random.random() / 2, False)
        else: 
            agent.move(direction_change, 1)
        agent.rays = env.get_state(agent.rays)
        renderer.draw(env.lines, agent)
