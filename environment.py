from sklearn.preprocessing import MinMaxScaler
import json
import math
import numpy as np
import concurrent.futures

class Environment:
    lines = []
    def __init__(self, filename, scale):
        shapes = []
        with open("polygons.json") as f:
            for f in json.load(f)["features"]:
                shapes.append(np.array(f["geometry"]["coordinates"]).flatten().reshape(-1, 2))
        self._create_scaler(scale, shapes)
        shapes = [self._coordinates_to_pixels(shape) for shape in shapes]
        for shape in shapes:
            for i in range(len(shape)-1):
                self.lines.append([
                    (shape[i][0], shape[i][1]), 
                    (shape[i+1][0], shape[i+1][1])
                ])

    def get_state(self, rays):
        processed_rays = []
        # for ray in rays:
        #     processed_rays.append(self.check_ray(ray, self.lines))
        with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
        #with concurrent.futures.ProcessPoolExecutor() as executor:
            #rays = executor.map(self.check_ray, rays)
            processed_ray = {executor.submit(self.check_ray, ray, self.lines): ray for ray in rays}
            for future in concurrent.futures.as_completed(processed_ray):
                #print(processed_ray[future])
                processed_rays.append(future.result())
        return sorted(processed_rays, key = lambda r: r.angle) 

    def check_ray(self, ray, lines):
        if ray.coords[1][1] > ray.coords[0][1]:
            lines = list(filter(lambda l: l[0][1] >= ray.coords[0][1] or l[1][1] >= ray.coords[0][1], lines))
        else:
            lines = list(filter(lambda l: l[0][1] < ray.coords[0][1] or l[1][1] < ray.coords[0][1], lines))
        if ray.coords[1][0] > ray.coords[0][0]:
            lines = list(filter(lambda l: l[0][0] > ray.coords[0][0] or l[1][0] > ray.coords[0][0], lines))
        else:
            lines = list(filter(lambda l: l[0][0] < ray.coords[0][0] or l[1][0] < ray.coords[0][0], lines))
        for line in lines:
            processed_ray = self.check_intersections(ray, line)
            if processed_ray.length < ray.length:
                return processed_ray
        return ray

    def check_intersections(self, ray, line):
        new_length = ray.length
        intersection = self.intersection2(
            (ray.coords[0][0]), 
            (ray.coords[0][1]), 
            (ray.coords[1][0]),
            (ray.coords[1][1]), 
            (line[0][0]),
            (line[0][1]), 
            (line[1][0]),
            (line[1][1])
        )
        if intersection:
            new_length = math.sqrt( ((ray.coords[0][0]-intersection[0])**2)+((ray.coords[0][1]-intersection[1])**2))
            if new_length < ray.length:
                ray.length = new_length
                ray.coords = (ray.coords[0], intersection)
        return ray


    def intersection2(self, Ax1, Ay1, Ax2, Ay2, Bx1, By1, Bx2, By2):
        d = (By2 - By1) * (Ax2 - Ax1) - (Bx2 - Bx1) * (Ay2 - Ay1)
        if d:
            uA = ((Bx2 - Bx1) * (Ay1 - By1) - (By2 - By1) * (Ax1 - Bx1)) / d
            uB = ((Ax2 - Ax1) * (Ay1 - By1) - (Ay2 - Ay1) * (Ax1 - Bx1)) / d
        else:
            return
        if not(0 <= uA <= 1 and 0 <= uB <= 1):
            return
        x = Ax1 + uA * (Ax2 - Ax1)
        y = Ay1 + uA * (Ay2 - Ay1)

        return x, y     

    def _create_scaler(self, scale, shapes):
        self.scaler = MinMaxScaler(feature_range=(0, scale))
        self.scaler.fit([list(p) for s in shapes for p in s])

    def _coordinates_to_pixels(self, shape):
        return self.scaler.transform(shape) 

    def _pixels_to_coordinates(self, shape):
        return self.scaler.inverse_transform(shape) 
