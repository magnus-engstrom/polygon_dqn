import math
import numpy as np
import random
from ray import Ray
class Agent:
    def __init__(self, position):
        self.speed = 1
        self.position = position
        self.old_position = position
        self.direction = random.random() * 6.28
        self.ray_count = 128
        self.fov = 0.45
        self.visibility = 300
        self.rays = self.cast_rays()

    def cast_rays(self):
        rays = []
        for i in range(0, self.ray_count):
            x = i / self.ray_count - 0.5
            angle = math.atan2(x, self.fov)
            rays.append(Ray(
                angle,
                self.visibility, 
                self.direction,
                self.position
            ))
        return rays

    def move(self, direction_change, speed, revert=False):
        self.speed = speed
        if revert: 
            self.position = self.old_position
        self.direction += direction_change
        self.rays = self.cast_rays()
        self.position = (
            self.position[0] + self.speed * math.cos(self.direction),
            self.position[1] + self.speed * math.sin(self.direction)
        )
        self.old_position = self.position