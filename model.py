from collections import deque
import random
import tensorflow as tf
from tensorflow.keras.layers import InputLayer, Dense
from tensorflow.keras import Sequential
from tensorflow.keras.optimizers import Adam
from tensorflow.keras.models import load_model
from tensorflow.keras import backend as K
from tensorflow.keras.layers import Activation
from tensorflow.keras.layers import BatchNormalization
import numpy as np
from tb import ModifiedTensorBoard

class Model:
    def __init__(self, n_actions, n_features):
        self.total_memory = deque(maxlen=100000)
        self.min_batch_samples = 200
        self.training_started = False
        self.epsilon = 1
        self.epsilon_decay = 0.995
        self.batch_size = 64
        self.model = None
        self.n_actions = n_actions
        self.discount = 0.99
        self.n_features = n_features
        self.training_count = 0
        self.name = "model_test_6"
        self.tensorboard_callback = ModifiedTensorBoard(self.name, log_dir="logs/{}".format(self.name))

    def store_memory_and_train(self, episode_memory, reward_per_step):
        self.total_memory += episode_memory
        print(len(self.total_memory), "rows in memory")
        if len(self.total_memory) >= self.batch_size * self.min_batch_samples:
            if not self.model:
                print("creating model")
                print((1, episode_memory[-1][0][0]))
                self.model = self.create_neural_network(self.n_features, self.n_actions)
                self.target_model = self.create_neural_network(self.n_features, self.n_actions)
                self.target_model.set_weights(self.model.get_weights())
            self.training_started = True
            self.tensorboard_callback.update_stats(
                reward_per_step=reward_per_step
            )
            self.__train()
            self.epsilon *= self.epsilon_decay
            print(self.training_count)
            if self.training_count % 10 == 0: 
                print("updating target model")
                self.target_model.set_weights(self.model.get_weights())
            self.training_count += 1
            print(self.epsilon)

    def predict_action(self, state):
        return np.argmax(self.model.predict(state.reshape(-1, len(state))))
        
    def __train(self):
        print("training on batch of size", self.batch_size)
        for old_state, action, new_state, reward, done in random.sample(self.total_memory, self.batch_size):
            if done:
                target = reward
            else:
                target = reward + self.discount * np.max(self.target_model.predict(new_state))
            target_vec = self.model.predict(old_state)[0]
            target_vec[action] = target
            self.model.fit(old_state, target_vec.reshape(-1, self.n_actions), epochs=1, verbose=0, callbacks=[self.tensorboard_callback])

    def create_neural_network(self, n_features, n_actions):
        model = Sequential()
        model.add(InputLayer(batch_input_shape=(1, n_features)))
        model.add(Dense(256, activation='relu'))
        model.add(Dense(256, activation='relu'))
        model.add(Dense(n_actions, activation='linear'))
        model.compile(loss="mse", optimizer=Adam(lr=0.0007), metrics=['accuracy'])
        return model