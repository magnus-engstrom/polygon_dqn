from sandbox_py import Env
import numpy as np
import tensorflow as tf

n_paths = 20
env_file = "gavle.json"
env = Env("sandbox/data/" + env_file, n_paths)

model_name = "model_test_233"
model = tf.keras.models.load_model("models/" + model_name + "_best")

max_steps = 2000

old_state = []
action = 0

geojson_features = []

for s in range(max_steps):
    for agent_id in range(n_paths):
        if env.agent_active(agent_id):
            if len(old_state) > 0:
                old_state = np.array(old_state)
                state_dataset = tf.data.Dataset.from_tensor_slices(old_state.reshape(-1, len(old_state))).batch(1)
                action = np.argmax(model.predict(state_dataset))
            (old_state, _, end) = env.step(action, agent_id)
            if s == max_steps-1 or end:
                geojson_features.append(env.agent_coordinates_path(agent_id))

f = open("generated_paths/" + model_name + ".json", "w")
f.write('{"type": "FeatureCollection", "features":[' + ','.join(geojson_features) + ']}')
f.close()
print("geojson saved")