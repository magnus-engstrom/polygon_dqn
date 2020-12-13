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
                ret.add(event.key)
    return ret


if __name__ == "__main__":
    polygons = [
        "gavle.json"
    ]
    random.seed(1)
    np.random.seed(1)
    env = Env("sandbox/data/" + random.choice(polygons))
    renderer = Renderer(500)
    n_actions = len(env.action_space)
    model = Model(n_actions)
    old_state = []
    render = False
    tagets_found = deque(maxlen=50)
    while True:
        keys = handle_input()
        if model.training_started and start_time is None:
            start_time = dt.datetime.today().timestamp()
        if pygame.K_r in keys: render = True
        action = model.predict_action(np.array(old_state), render)
        (state, reward, end) = env.step(action)
        state[0] /= 3.14 # scale target bearing to between -1 to +1
        target_bearing, target_distance, can_see_target, *_ = state
        if len(state) > 1: old_state = state
        if end or (render and env.get_agent_targets_count() > 10.0):
            if not render:
                tagets_found.append(env.get_agent_targets_count())
                model.store_memory_and_train(
                    [
                        [
                            np.array(d["old_state"]).reshape(-1, len(d["old_state"])),
                            d["action"],
                            np.array(d["new_state"]).reshape(-1, len(d["new_state"])),
                            d["reward"],
                            d["done"],
                        ] for d in [dict(m) for m in env.agent_memory()]
                    ],
                    env.get_agent_targets_count(),
                    sum(tagets_found) / len(tagets_found)
                )
            env.reset()
            old_state = []
            agg_reward = 0
            render = False
            continue
        if render:
            renderer.draw(
                env.lines, 
                env.get_agent_rays(), 
                env.targets, 
                target_bearing*3.14, 
                target_distance, 
                reward,
                list(env.agent_closest_target), 
                can_see_target,
                env.agent_past_position,
                env.agent_collected_targets, 
            )
        
        if start_time is not None:
            i += 1
            time_diff = dt.datetime.today().timestamp() - start_time
            if i % 100 == 0: 
                print("- - -")
                print(i / time_diff)
                print("- - -")

