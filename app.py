import pygame
from renderer import Renderer
from environment import Environment
from agent import Agent
import random

env = Environment("polygons.json", 500)
renderer = Renderer(500)

agent = Agent((250, 10), 2)
distance_to_wall = 1000
direction_change = 0

while True:
    # if distance_to_wall < 20:
    #     direction_change += random.random() - 0.5
    # else: 
    direction_change = 0
    agent.move(direction_change)
    agent.rays, distance_to_wall, ray_lengths = env.get_state(agent.rays, agent.visibility)
    renderer.draw(env.lines, agent, ray_lengths)
