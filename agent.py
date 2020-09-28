import math
import numpy as np
from ray import Ray
class Agent:
    def __init__(self, position, direction):
        self.position = position
        self.direction = direction
        self.ray_count = 80
        self.fov = 110 * math.pi / 180
        self.visibility = 200
        self.rays = self.cast_rays()

    def cast_rays(self):
        rays = []
        #angle_list = [math.atan(x / self.visibility) for x in range(-self.ray_count // 2, self.ray_count // 2)]
        #for i in angle_list:

        for i in np.arange(-1.2, 1.2, 0.03):
            rays.append(Ray(
                i, 
                self.visibility, 
                self.fov/self.ray_count,
                self.direction,
                self.position
            ))
        return rays

    def move(self, direction_change):
        self.direction += direction_change
        self.rays = self.cast_rays()
        self.position = (
            self.position[0] + 1 * math.cos(self.direction),
            self.position[1] + 1 * math.sin(self.direction)
        )
        