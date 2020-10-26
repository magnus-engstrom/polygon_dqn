from renderer import Renderer
from sandbox_py import Env
from model import Model
import numpy as np
import random

import datetime as dt

start_time = dt.datetime.today().timestamp()
i = 0
if __name__ == "__main__":
    env = Env("sandbox/data/polygons.json")
    renderer = Renderer(500)
    env_lines = env.lines
    rays = [999]
    n_actions = len(env.action_space)
    model = Model(n_actions)
    episode_memory = []
    old_state = None
    agg_reward = 0
    render = False
    render_countdown = 5
    while True:
        if (not render or agg_reward < -3.0) and (random.uniform(0,1) <= model.epsilon or not model.training_started or not old_state):
            # random movement
            action = random.randint(0, n_actions-1)
        else:
            # model movement
            if not old_state:
                action = 1
            else:
                action = model.predict_action(np.array(old_state))
        
        (state, reward, end) = env.step(action)
        if not end:
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
                model.store_memory_and_train(episode_memory)
            episode_memory = []
            old_state = None
            agg_reward = 0
            if render_countdown < 1:
                render = True
                render_countdown = 5
            else:
                render = False
            render_countdown -= 1
            continue

        target_distance, target_bearing, *rays = state
        if render:
            renderer.draw(env_lines, env.get_agent_rays(), env.targets, target_bearing, target_distance)

        time_diff = dt.datetime.today().timestamp() - start_time
        i += 1
        if i % 100 == 0: print(i / time_diff)