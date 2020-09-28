import pygame
from renderer import Renderer
from environment import Environment
from agent import Agent
import random

env = Environment("polygons.json", 1000)
renderer = Renderer(500)

agent = Agent((250, 250))
distance_to_wall = 1000
direction_change = 0

while True:
    if min([r.length for r in agent.rays]) < 10:
        agent.move(4 * random.random() - 2, True)
    else: 
        agent.move(direction_change)
    agent.rays = env.get_state(agent.rays)
    renderer.draw(env.lines, agent)
