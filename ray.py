import math

class Ray:
    def __init__(self, angle, length, player_angle, player_position):
        self.angle = angle
        self.length = length
        self.max_length = length
        self.player_angle = player_angle
        self.coords = (
            player_position,
            (
                player_position[0] + length * math.cos(player_angle + self.angle),
                player_position[1] + length * math.sin(player_angle + self.angle)
            )
        )