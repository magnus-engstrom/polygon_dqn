import math
import numpy as np
from ray import Ray
class Agent:
    def __init__(self, position, direction):
        self.speed = 1
        self.position = position
        self.direction = direction
        self.ray_count = 30
        self.fov = 0.4
        self.visibility = 300
        self.rays = self.cast_rays()

    def cast_rays(self):
        rays = []
        #angle_list = [math.atan(x / self.visibility) for x in range(-self.ray_count // 2, self.ray_count // 2)]
        #for i in angle_list:

        #for i in np.arange(-self.fov/2, self.fov/2, self.fov/self.ray_count):
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

        #ray = map.cast(player, self.direction + angle, this.range)

    def move(self, direction_change):
        self.direction += direction_change
        self.rays = self.cast_rays()
        self.position = (
            self.position[0] + self.speed * math.cos(self.direction),
            self.position[1] + self.speed * math.sin(self.direction)
        )
        