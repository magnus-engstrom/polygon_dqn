from sandbox_py import Env
import numpy as np
import tensorflow as tf
import random
from multiprocessing import Process
#from renderer import Renderer

#renderer = Renderer(500)
n_paths = 5
max_steps = 5000
env_file = "exp1.json"
env = Env("sandbox/data/" + env_file, n_paths, max_steps)

model_name = "model_117"
model = tf.keras.models.load_model("models/" + model_name + "_best")

old_state = []
action = 0

geojson_features = []

print("targets: ", len(env.targets))

for agent_id in range(n_paths):
    print("processing agent: ", agent_id)
    for s in range(max_steps):
        if env.agent_active(agent_id):
            render = False
            if len(old_state) > 0:
                old_state = np.array(old_state)
                if random.uniform(0,1) > 0.98:
                    action = random.randint(0,  len(env.action_space(0))-1)
                else:
                    state_dataset = tf.data.Dataset.from_tensor_slices(old_state.reshape(-1, len(old_state))).batch(1)
                    action = np.argmax(model.predict(state_dataset))
            (old_state, reward, end) = env.step(action, agent_id)
            if len(env.targets) == env.agent_targets_count(agent_id):
                print(env.agent_targets_count(agent_id))
                end = True
            if s == max_steps-1 or end:
                if env.agent_targets_count(agent_id) == len(env.targets):
                    geojson_features.append(env.agent_coordinates_path(agent_id))
                break
            # if render:
            #     target_bearing, *_ = old_state
            #     renderer.draw(
            #         env.lines, 
            #         env.agent_rays(agent_id), 
            #         env.targets, 
            #         target_bearing, 
            #         0.0, 
            #         reward,
            #         list(env.agent_closest_target(agent_id)), 
            #         0.0,
            #         env.agent_past_position(agent_id),
            #         env.agent_collected_targets(agent_id), 
            #         1 / 1,
            #         env.agent_age(agent_id),
            #     )

f = open("generated_paths/" + model_name + ".json", "w")
f.write('{"type": "FeatureCollection", "features":[' + ','.join(geojson_features) + ']}')
f.close()
print("geojson saved")