import math

class Ray:
    def __init__(self, angle, length, step_size, player_angle, player_position):
        self.angle = angle
        self.length = length
        self.max_length = length
        self.player_angle = player_angle
        self.step_size = step_size
        self.coords = (
            player_position,
            (
                player_position[0] + length * math.cos(player_angle +self.angle),
                player_position[1] + length * math.sin(player_angle + self.angle)
            )
        )

    def scale_ration(self):
        if self.length == self.max_length:
            return 1
        return self.lens_length() / self.max_length

    def lens_length(self):
        #print(self.length * math.cos(self.angle) / 500)
        return self.length * math.cos(self.angle)
        #800/(rayDistMath.abs(Math.cos(angleOff))); 