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

class Model:
    def __init__(self, n_actions):
        self.total_memory = deque(maxlen=50000)
        self.min_batch_samples = 10
        self.training_started = False
        self.epsilon = 0.9
        self.epsilon_decay = 0.995
        self.batch_size = 64
        self.model = None
        self.n_actions = n_actions
        self.discount = 0.98
        self.n_features = 34
        self.training_count = 0

    def store_memory_and_train(self, episode_memory):
        self.total_memory += episode_memory
        print(len(self.total_memory), "rows in memory")
        if len(self.total_memory) >= self.batch_size * self.min_batch_samples:
            if not self.model:
                print("creating model")
                print((1, len(episode_memory[0][0])))
                self.model = self.create_neural_network(self.n_features, self.n_actions)
                self.target_model = self.create_neural_network(self.n_features, self.n_actions)
                self.target_model.set_weights(self.model.get_weights())
            self.training_started = True
            self.__train()
            self.epsilon *= self.epsilon_decay
            print(self.training_count)
            if self.training_count % 20 == 0: 
                print("updating target model")
                self.target_model.set_weights(self.model.get_weights())
            self.training_count += 1

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
            self.model.fit(old_state, target_vec.reshape(-1, self.n_actions), epochs=1, verbose=0)

    def create_neural_network(self, n_features, n_actions):
        model = Sequential()
        model.add(InputLayer(batch_input_shape=(1, n_features)))
        model.add(Dense(512, activation='relu'))
        model.add(Dense(512, activation='relu'))
        model.add(Dense(n_actions, activation='linear'))
        model.compile(loss="mse", optimizer=Adam(lr=0.001), metrics=['accuracy'])
        return model