import pygame
from renderer import Renderer
from environment import Environment
from agent import Agent
import random

env = Environment("polygons.json", 500)
renderer = Renderer(500)

agent = Agent((250, 250), 2)
distance_to_wall = 1000
direction_change = 0

while True:
    if min([r.length for r in agent.rays]) < 25:
        direction_change += random.random() - 0.5
    else: 
        direction_change = 0
    agent.move(direction_change)
    agent.rays = env.get_state(agent.rays)
    renderer.draw(env.lines, agent)
