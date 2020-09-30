import pygame
import math
import os
class Renderer:
    def __init__(self, res):
        pygame.init()
        self.scale = 500
        self.agent_visibility = 0.1
        self.frame_count = 0
        self.assets = {}
        self.display=pygame.display.set_mode((500 + 500, res))
        self.clock = pygame.time.Clock()
        self.prepare_assets_3D()
        for event in pygame.event.get():
            if event.type == pygame.MOUSEBUTTONUP:
                pos = pygame.mouse.get_pos()

    def prepare_assets_3D(self):
        floor = pygame.image.load(os.path.join("assets/floor.png"))
        floor.convert()
        self.assets["floor"] = pygame.transform.scale(floor, (500, 150))
        floor = pygame.image.load(os.path.join("assets/sky.png"))
        floor.convert()
        self.assets["sky"] = pygame.transform.scale(floor, (500, 150))

    def draw_3D(self, ray_line_strings):
        offset = 500
        width = 500 / len(ray_line_strings)
        screen_height = 300
        shading = 0
        color_max = 150
        self.display.blit(self.assets["sky"], [500, 0, 500, 150])
        self.display.blit(self.assets["floor"], [500, 150, 500, 150])
        for i, ray_line_string in enumerate(ray_line_strings):
            for ray_line in ray_line_string:
                #if not ray.max_length == ray.length:
                z = ray_line[4] * math.cos(ray_line[5])
                wall_height = screen_height / z * 15
                wall_height = min(wall_height, 300)
                top = (screen_height / 2) - (wall_height / 2)
                shading = color_max * (1 - z/self.agent_visibility)
                rect = [i + offset, top, width + 1, wall_height]
                pygame.draw.rect(self.display, (shading, shading, shading), rect)
                offset += width - 1

    def draw_2D(self, env_line_strings, ray_line_strings):
        for env_line_string in env_line_strings:
            for env_line in env_line_string:
                start = (env_line[0] * self.scale, env_line[1] * self.scale)
                end = (env_line[2] * self.scale, env_line[3] * self.scale)
                pygame.draw.line(self.display, (200, 200, 200), start, end)
        for ray_line_string in ray_line_strings:
            for ray_line in ray_line_string:
                start = (ray_line[0] * self.scale, ray_line[1] * self.scale)
                end = (ray_line[2] * self.scale, ray_line[3] * self.scale)
                pygame.draw.line(self.display, (255, 0, 0), start, end)
            #pygame.draw.circle(self.display, (0, 255, 0), (agent.position[0], agent.position[1]), 5)

    def draw(self, env_line_strings, ray_line_strings):
        self.display.fill((0, 0, 0))
        self.draw_2D(env_line_strings, ray_line_strings)
        self.draw_3D(ray_line_strings)
        pygame.display.update()
        pygame.display.flip()
        self.clock.tick(60)
        self.frame_count += 1
        return self.frame_count 
