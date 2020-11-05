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
        tf.random.set_seed(1)
        random.seed(1)
        np.random.seed(1)
        self.total_memory = deque(maxlen=100000)
        self.min_batch_samples = 100
        self.training_started = False
        self.epsilon = 1
        self.epsilon_decay = 0.997
        self.min_epsilon = 0.001
        self.batch_size = 128
        self.model = None
        self.n_actions = n_actions
        self.discount = 0.995
        self.n_features = n_features
        self.training_count = 0
        self.name = "model_test_118"
        self.min_learning_rate = 0.0004
        self.learning_rate = 0.0015
        self.update_counter = 0
        self.update_model_at = 10
        self.tensorboard_callback = ModifiedTensorBoard(self.name, log_dir="logs/{}".format(self.name))

    def store_memory_and_train(self, episode_memory, reward_per_step, targets_found):
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
                reward_per_step=reward_per_step,
                targets_found=targets_found,
                update_counter=self.update_counter,
                learning_rate=self.learning_rate,
                epsilon=self.epsilon
            )
            self.__train()
            if self.epsilon_decay > self.min_epsilon:
                self.epsilon *= self.epsilon_decay
            self.update_counter += 1
            if self.update_counter == self.update_model_at: 
                self.update_counter = 0
                print("### updating target model ###")
                self.target_model.set_weights(self.model.get_weights())
                if self.learning_rate > self.min_learning_rate:
                    self.learning_rate *= 0.99
                    K.set_value(self.model.optimizer.learning_rate, self.learning_rate)
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
            self.model.fit(old_state, target_vec.reshape(-1, self.n_actions), epochs=1, verbose=0, callbacks=[self.tensorboard_callback])

    def create_neural_network(self, n_features, n_actions):
        model = Sequential()
        model.add(InputLayer(batch_input_shape=(1, n_features)))
        model.add(Dense(256, activation='relu'))
        model.add(Dense(256, activation='relu'))
        model.add(Dense(n_actions, activation='linear'))
        model.compile(loss="mse", optimizer=Adam(lr=self.learning_rate), metrics=['accuracy'])
        return model