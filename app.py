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
                print(event.key)
                ret.add(event.key)
    return ret


if __name__ == "__main__":
    polygons = [
        "polygons2.json",
        "rooms.json",
        "polygons_hard.json",
        "room_hard.json",
        "complex.json"
    ]
    random.seed(1)
    np.random.seed(1)
    env = Env("sandbox/data/" + random.choice(polygons))
    renderer = Renderer(500)
    rays = [999]
    n_actions = len(env.action_space)
    model = Model(n_actions, int(env.ray_count+3))
    episode_memory = []
    old_state = []
    agg_reward = 0
    render = False
    agent_positions = deque(maxlen=50)
    rewards = deque(maxlen=50)
    tagets_found = deque(maxlen=50)
    dump_log = False
    paused = False
    while True:
        keys = handle_input()
        if 107 in keys:
            dump_log = False
        if pygame.K_l in keys:
            dump_log = True
        if 112 in keys:
            paused = True
        if 111 in keys:
            paused = False
        if paused:
            continue
        if not paused:
            if model.training_started and start_time is None:
                start_time = dt.datetime.today().timestamp()
            if pygame.K_r in keys:
                render = True
            action = model.predict_action(np.array(old_state), render)
            (state, reward, end) = env.step(action)
            if not env.agent_active:
                end = True
            agg_reward += reward
            state[0] /= 3.14
            if dump_log:
                print("#### Logging ###")
                print("state", state)
                print("actions", model.model.predict(np.array(state).reshape(-1, len(state))))
                print("### end ###")
            if len(old_state) > 0 and env.agent_active:
                episode_memory.append([
                    np.array(old_state).reshape(-1, len(old_state)),
                    action, 
                    np.array(state).reshape(-1, len(state)), 
                    reward,
                    end
                ])
            target_bearing, target_distance, can_see_target, *rays = state
            if len(state) > 1:
                old_state = state
            if end:
                print("total reward", agg_reward)
                if not render:
                    rewards.append(agg_reward)
                    tagets_found.append(env.get_agent_targets_count())
                    agent_positions.append(list(env.agent_position))
                    model.store_memory_and_train(
                        episode_memory, 
                        agg_reward/len(episode_memory), 
                        env.get_agent_targets_count(),
                        sum(tagets_found) / len(tagets_found),
                        sum(rewards) / len(rewards)
                    )
                if random.uniform(0,1) > 0.9: 
                    env = Env("sandbox/data/" + random.choice(polygons))
                    agent_positions = deque(maxlen=50)
                env.reset()
                rays = [999]
                episode_memory = []
                old_state = []
                agg_reward = 0
                render = False
                continue
            if render:
                renderer.draw(env.lines, env.get_agent_rays(), env.targets, target_bearing*3.14, 
                    target_distance, reward, agg_reward, agent_positions, list(env.agent_closest_target), can_see_target
                )
            
            if start_time is not None:
                i += 1
                time_diff = dt.datetime.today().timestamp() - start_time
                if i % 100 == 0: 
                    print("- - -")
                    print(i / time_diff)
                    print("- - -")

        