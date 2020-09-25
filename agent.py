import math
import numpy as np
class Agent:
    def __init__(self, position, direction):
        self.position = position
        self.direction = direction
        self.rays = []
        self.visibility = 500

    def move(self, direction_change):
        self.direction += direction_change
        self.rays = [self.cast_ray(i) for i in np.arange(-1.2, 1.2, 0.03)]
        self.position = (
            self.position[0] + 1 * math.cos(self.direction),
            self.position[1] + 1 * math.sin(self.direction)
        )

    def cast_ray(self, ray_direction=0):
        return [
            self.position,
            (
                self.position[0] + self.visibility * math.cos(self.direction + ray_direction),
                self.position[1] + self.visibility * math.sin(self.direction + ray_direction)
            )
        ]
        