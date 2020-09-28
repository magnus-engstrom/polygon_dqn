import pygame
import math
import os
class Renderer:
    def __init__(self, res):
        pygame.init()
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

    def draw_3D(self, rays):
        offset = 500
        width = 500 / len(rays)
        screen_height = 300
        shading = 0
        color_max = 150
        self.display.blit(self.assets["sky"], [500, 0, 500, 150])
        self.display.blit(self.assets["floor"], [500, 150, 500, 150])
        for i, ray in enumerate(rays):
            #if not ray.max_length == ray.length:
            z = ray.length * math.cos(ray.angle)
            wall_height = screen_height / z * 15
            wall_height = min(wall_height, 300)
            top = (screen_height / 2) - (wall_height / 2)
            shading = color_max * (1 - z/ray.max_length)
            rect = [i + offset, top, width + 1, wall_height]
            pygame.draw.rect(self.display, (shading, shading, shading), rect)
            offset += width - 1

    def draw_2D(self, lines, agent):
        for line in lines:
            pygame.draw.line(self.display, (200, 200, 200), line[0], line[1])
            pygame.draw.circle(self.display, (0, 255, 0), agent.position, 10)
            for ray in agent.rays:
                pygame.draw.line(self.display, (255, 0, 0), ray.coords[0], ray.coords[1]) 

    def draw(self, lines, agent):
        self.display.fill((0, 0, 0))
        self.draw_2D(lines, agent)
        self.draw_3D(agent.rays)
        pygame.display.update()
        pygame.display.flip()
        self.clock.tick(10)
        self.frame_count += 1
        return self.frame_count 
