from renderer import Renderer
from sandbox_py import Env
from model import Model
import numpy as np
import random
import pygame

import datetime as dt

start_time = dt.datetime.today().timestamp()
i = 0

def handle_input():
    ret = set()
    for event in pygame.event.get():
        if event.type == pygame.KEYDOWN or event.type == pygame.KEYUP:
            if pygame.key.get_focused():
                if event.key == pygame.K_r:
                    ret.add(event.key)
    return ret

if __name__ == "__main__":
    env = Env("sandbox/data/polygons.json")
    renderer = Renderer(500)
    env_lines = env.lines
    rays = [999]
    n_actions = len(env.action_space)
    model = Model(n_actions, int(env.ray_count+2))
    episode_memory = []
    old_state = None
    agg_reward = 0
    render = False
    while True:
        keys = handle_input()
        if pygame.K_r in keys:
            render = True
        if (render or random.uniform(0,1) <= model.epsilon) and model.training_started and old_state != None and agg_reward < -10:
            action = model.predict_action(np.array(old_state))
        else:
            action = random.randint(0, n_actions-1)
        (state, reward, end) = env.step(action)
        agg_reward += reward
        if old_state:
            episode_memory.append([
                np.array(old_state).reshape(-1, len(old_state)), 
                action, 
                np.array(state).reshape(-1, len(state)), 
                reward, 
                end
            ])
        old_state = state
        if end:
            env.reset()
            rays = [999]
            print("total reward", agg_reward)
            if not render:
                model.store_memory_and_train(episode_memory, agg_reward/len(episode_memory))
            episode_memory = []
            old_state = None
            agg_reward = 0
            render = False
            if i % 20 == 0:
                render = True
            continue

        target_distance, target_bearing, *rays = state
        if render:
            renderer.draw(env_lines, env.get_agent_rays(), env.targets, target_bearing, target_distance, reward, agg_reward)

        time_diff = dt.datetime.today().timestamp() - start_time
        i += 1
        if i % 100 == 0: print(i / time_diff)

        