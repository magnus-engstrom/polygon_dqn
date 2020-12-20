from collections import deque
import random
import tensorflow as tf
from tensorflow.keras.layers import InputLayer, Dense
from tensorflow.keras import Sequential
from tensorflow.keras.optimizers import Adam
from tensorflow.keras import backend as K
from tensorflow.keras.layers import Activation
import numpy as np
from tb import ModifiedTensorBoard

class Model:
    def __init__(self, n_actions):
        tf.random.set_seed(1)
        random.seed(1)
        np.random.seed(1)
        self.total_memory = deque(maxlen=250000)
        self.min_batch_samples = 5
        self.training_started = False
        self.epsilon = 1
        self.epsilon_decay = 0.998
        self.min_epsilon = 0.01
        self.batch_size = 64
        self.model = None
        self.n_actions = n_actions
        self.discount = 0.997
        self.name = "model_test_231"
        self.min_learning_rate = 0.00002
        self.learning_rate = 0.0005
        self.mean_targets_found = 0
        self.max_mean_targets_found = 0
        self.tensorboard_callback = ModifiedTensorBoard(self.name, log_dir="logs/{}".format(self.name))

    def store_memory_and_train(self, episode_memory, targets_found, mean_targets_found):
        self.total_memory += episode_memory
        print(len(self.total_memory), "rows in memory")
        if len(self.total_memory) >= self.batch_size * self.min_batch_samples:
            if not self.model:
                print("creating model")
                print((1, episode_memory[-1][0][0]))
                self.model = self.__create_neural_network(len(episode_memory[-1][0][0]), self.n_actions)
                self.target_model = self.__create_neural_network(len(episode_memory[-1][0][0]), self.n_actions)
                self.target_model.set_weights(self.model.get_weights())
            self.training_started = True
            self.tensorboard_callback.update_stats(
                targets_found=targets_found,
                learning_rate=self.learning_rate,
                epsilon=self.epsilon,
                mean_targets_found = mean_targets_found
            )
            self.__train()
            if self.mean_targets_found <= mean_targets_found:
                self.model.save("models/" + self.name + "_latest")
                if self.epsilon > self.min_epsilon:
                    self.epsilon *= self.epsilon_decay
                if self.learning_rate > self.min_learning_rate:
                    self.learning_rate *= 0.999
                    K.set_value(self.model.optimizer.learning_rate, self.learning_rate)
            else:
                if self.epsilon < 0.7:
                    self.epsilon *= 2 - self.epsilon_decay
                    print("increase epsilon")
            self.mean_targets_found = mean_targets_found
            if self.max_mean_targets_found < self.mean_targets_found:
                self.model.save("models/" + self.name + "_best")
                self.max_mean_targets_found = self.mean_targets_found

    def predict_action(self, state, no_exploration):
            if (no_exploration or random.uniform(0,1) > self.epsilon) and len(state) > 0 and random.uniform(0,1) > self.min_epsilon:
                return np.argmax(self.model.predict(state.reshape(-1, len(state))))
            else:
                return random.randint(0, self.n_actions-1)
        
    def __train(self):
        print("training on batch of size", self.batch_size)
        batch_losses = []
        for old_state, action, new_state, reward, done in random.sample(self.total_memory, self.batch_size):
            if done:
                target = reward
            else:
                target = reward + self.discount * np.max(self.target_model.predict(new_state))
            target_vec = self.model.predict(old_state)[0]
            target_vec[action] = target
            loss = self.model.fit(old_state, target_vec.reshape(-1, self.n_actions), epochs=1, verbose=0, callbacks=[self.tensorboard_callback])
            batch_losses.append([loss.history['loss'], [old_state, action, new_state, reward, done]])
            TAU = 1.0/2500.0
            for t, e in zip(self.target_model.trainable_variables, self.model.trainable_variables):
                        t.assign(t * (1 - TAU) + e * TAU)
        batch_losses = sorted(batch_losses, key=lambda x: x[0], reverse=True)
        for sample in batch_losses[:int(self.batch_size / 10)]:
            self.total_memory.append(sample[1])
            print("prioritized memory: ", sample[0], sample[1][-2], sample[1][-1])
        print(target_vec)

    def __create_neural_network(self, n_features, n_actions):
        def huber_loss(a, b, in_keras=True):
            error = a - b
            quadratic_term = error*error / 2
            linear_term = abs(error) - 1/2
            use_linear_term = (abs(error) > 1.0)
            if in_keras:
                use_linear_term = K.cast(use_linear_term, 'float32')
            return use_linear_term * linear_term + (1-use_linear_term) * quadratic_term
        model = Sequential()
        model.add(InputLayer(batch_input_shape=(1, n_features)))
        model.add(Dense(256, activation='relu'))
        model.add(Dense(256, activation='relu'))
        model.add(Dense(n_actions, activation='linear'))
        model.compile(loss=huber_loss, optimizer=Adam(lr=self.learning_rate), metrics=['accuracy'])
        return model