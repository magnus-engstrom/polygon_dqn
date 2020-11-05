from renderer import Renderer
from sandbox_py import Env
from model import Model
import numpy as np
import random
import pygame
from collections import deque

import datetime as dt

start_time = None
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
    random.seed(1)
    np.random.seed(1)
    env = Env("sandbox/data/polygons2.json")
    renderer = Renderer(500)
    env_lines = env.lines
    rays = [999]
    n_actions = len(env.action_space)
    model = Model(n_actions, int(env.ray_count+2))
    episode_memory = []
    old_state = []
    agg_reward = 0
    render = False
    agent_positions = deque(maxlen=50)
    while True:
        if model.training_started and start_time is None:
            start_time = dt.datetime.today().timestamp()
        keys = handle_input()
        if pygame.K_r in keys:
          render = True
        if (render or random.uniform(0,1) < model.epsilon) and model.training_started and len(old_state) > 1:
            model.predict_action(np.array(old_state))
        else:
            action = random.randint(0, n_actions-1)
        (state, reward, end) = env.step(action)
        if env.agent_age > 500:
            end = True
        agg_reward += reward
        state[1] = (state[1] + 3.14) / 6.28
        if len(old_state) > 0:
            #store_short_term_memory()
            episode_memory.append([
                old_state, 
                action, 
                np.array(state).reshape(-1, len(state)), 
                reward, 
                end
            ])
        target_distance, target_bearing, *rays = state
        if len(state) > 1:
            old_state = np.array(state).reshape(-1, len(state))
        if end:
            agent_positions.append(list(env.agent_position))
  
            print("total reward", agg_reward)
            if not render:
                model.store_memory_and_train(episode_memory, agg_reward/len(episode_memory), env.get_agent_targets_count())
            env.reset()
            rays = [999]
            episode_memory = []
            old_state = []
            agg_reward = 0
            render = False
            continue
        if render:
            renderer.draw(env_lines, env.get_agent_rays(), env.targets, target_bearing, 
                target_distance, reward, agg_reward, agent_positions, list(env.agent_closest_target)
            )
        
        if start_time is not None:
            i += 1
            time_diff = dt.datetime.today().timestamp() - start_time
            if i % 100 == 0: 
                print("- - -")
                print(i / time_diff)
                print("- - -")

        