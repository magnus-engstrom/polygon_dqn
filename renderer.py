import pygame

class Renderer:
    def __init__(self, res):
        pygame.init()
        self.frame_count = 0
        self.display=pygame.display.set_mode((res*2, res))
        self.clock = pygame.time.Clock()
        for event in pygame.event.get():
            if event.type == pygame.MOUSEBUTTONUP:
                pos = pygame.mouse.get_pos()

    def draw_3D(self, rays, ray_lengths):
        offset = 500
        width = 500 / len(rays)
        screen_height = 500
        shading = 0
        color_max = 150
        visibility = 500
        for i, ray in enumerate(rays):
            height = 500 * ((visibility - ray_lengths[i]) / visibility)
            shading = color_max * ((visibility - ray_lengths[i]) / visibility)

            y_start = (screen_height / 2) - (height / 2)
            rect = [i + offset, y_start, width+2, height]
            pygame.draw.rect(self.display, (shading, shading, shading), rect)
            offset += width

    def draw_2D(self, lines, agent):
        for line in lines:
            pygame.draw.line(self.display, (200, 200, 200), line[0], line[1])
            pygame.draw.circle(self.display, (0, 255, 0), agent.position, 10)
            for line in agent.rays:
                pygame.draw.line(self.display, (255, 0, 0), line[0], line[1]) 

    def draw(self, lines, agent, ray_lengths):
        self.display.fill((0, 0, 0))
        self.draw_2D(lines, agent)
        self.draw_3D(agent.rays, ray_lengths)
        pygame.display.update()
        self.clock.tick(10)
        self.frame_count += 1
        return self.frame_count 
