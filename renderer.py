import pygame
import math
import os
class Renderer:
    def __init__(self, res):
        pygame.init()
        self.scale = 500
        self.agent_visibility = 0.6
        self.frame_count = 0
        self.assets = {}
        self.display=pygame.display.set_mode((500 + 500, res))
        self.clock = pygame.time.Clock()
        self.prepare_assets_3D()
        pygame.font.init() # you have to call this at the start, 
                   # if you want to use this module.
        self.font_display = pygame.font.SysFont('Arial', 50)
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

    def draw_3D(self, rays, target_bearing, target_distance, can_see_target):
        rays = list(filter(lambda x: x["in_fov"] > 0, rays))
        offset = 500
        screen_width = 500
        width = screen_width / len(rays)
        screen_height = 300
        shading = 0
        color_max = 150
        target_x = -1
        self.display.blit(self.assets["sky"], [500, 0, 500, 150])
        self.display.blit(self.assets["floor"], [500, 150, 500, 150])
        for i, ray in enumerate(rays):
            z = ray["length"] * math.cos(ray["angle"])
            wall_height = screen_height / z * 0.015
            wall_height = min(wall_height, screen_height)
            top = (screen_height / 2) - (wall_height / 2)
            shading = color_max * (1 - z/self.agent_visibility)
            rect = [i + offset, top, width + 1, wall_height]
            pygame.draw.rect(self.display, (shading, shading, shading), rect)
            offset += width - 1

        if target_bearing >= rays[0]["angle"] and target_bearing <= rays[-1]["angle"] and can_see_target > 0:
            z = target_distance * math.cos(target_bearing)
            wall_height = screen_height / z * 0.015
            wall_height = min(wall_height, screen_height)
            top = (screen_height / 2) - (wall_height / 2)
            target_x = int((0.5 - (target_bearing / abs(rays[0]["angle"] - rays[-1]["angle"]))) * screen_width)
            shading = color_max * (1 - z/self.agent_visibility)
            for j in range(int((width*2)/z*0.1)):
                if int(target_x/width)+j < len(rays) and rays[int(target_x/width)+j]["length"] > target_distance:
                    rect = [screen_width + target_x + j, top, 1, wall_height]
                    pygame.draw.rect(self.display, (int(shading/2), shading+30, int(shading/2)), rect)

    def draw_2D(self, env_lines, rays, targets, closest_target, past_position, collected_targets):
        resize = 0.4
        ax = (rays[0]["start_x"] * self.scale) * resize
        ay = (rays[0]["start_y"] * self.scale) * resize
        for env_line in env_lines:
            start = ((env_line["start_x"] * self.scale * resize - ax + (self.scale / 2)), env_line["start_y"] * self.scale * resize - ay + (self.scale / 2))
            end = (env_line["end_x"] * self.scale * resize - ax + (self.scale / 2), env_line["end_y"] * self.scale * resize - ay + (self.scale / 2))
            pygame.draw.line(self.display, (200, 200, 200), start, end)
        for target in targets:
            pygame.draw.circle(self.display, (50, 150, 50), [target["x"] * self.scale * resize - ax + self.scale / 2, target["y"] * self.scale * resize - ay + self.scale / 2], 5)
        for ray in rays:
            start = (self.scale / 2, self.scale / 2)
            end = ((ray["end_x"] * self.scale * resize - ax + self.scale / 2) , ray["end_y"] * self.scale * resize - ay + self.scale / 2)
            if ray["in_fov"] > 0:
                pygame.draw.line(self.display, (150 + 50* (1-(ray["length"] / ray["max_length"]))*2, 100+25*(1-(ray["length"] / ray["max_length"])), 0), start, end)
            else:
                pygame.draw.line(self.display, (50, 50, 50), start, end)

        pygame.draw.circle(self.display, (200, 200, 0), (self.scale / 2, self.scale / 2), 5)
        pygame.draw.circle(self.display, (200, 200, 0), (past_position[0] * self.scale * resize - ax + self.scale / 2, past_position[1] * self.scale * resize - ay + self.scale / 2), 5, 1)
        pygame.draw.line(self.display, (100, 100, 100), 
            (self.scale / 2, self.scale / 2), 
            (closest_target[0] * self.scale * resize - ax + self.scale / 2, closest_target[1] * self.scale * resize - ay + self.scale / 2)
        )
        for ct in collected_targets:
            pygame.draw.circle(self.display, (50, 50, 0), (ct[0] * self.scale * resize - ax + self.scale / 2, ct[1] * self.scale * resize - ay + self.scale / 2), 5)
        

    def draw_reward(self, reward, target_bearing):
        textsurface = self.font_display.render(str(reward), False, (255, 255, 255))
        self.display.blit(textsurface,(650,350))
        textsurface = self.font_display.render(str(target_bearing), False, (150, 150, 150))
        self.display.blit(textsurface,(650,460))

    def draw(self, env_lines, rays, targets, target_bearing, target_distance, reward, closest_target, can_see_target, past_position, collected_targets):
        self.display.fill((10, 10, 10))
        self.draw_2D(env_lines, rays, targets, closest_target, past_position, collected_targets)
        self.draw_3D(rays, target_bearing, target_distance, can_see_target)
        self.draw_reward(reward, target_bearing)
        pygame.display.update()
        #pygame.display.flip()
        self.clock.tick(22)
        self.frame_count += 1
        return self.frame_count



