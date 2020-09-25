from sklearn.preprocessing import MinMaxScaler
import json
import math
import numpy as np
from shapely.geometry import LineString

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

    def get_state(self, rays, visibility):
        processed_rays = []
        min_distance = visibility
        ray_lengths = []
        for ray in rays:
            state_ray = ray
            ray_length = visibility
            new_length = visibility
            for line in self.lines:
                intersection = self.intersection(LineString(ray), LineString(line), visibility)
                if intersection:
                    new_length = math.sqrt( ((ray[0][0]-intersection[0])**2)+((ray[0][1]-intersection[1])**2))
                    if new_length < ray_length:
                        ray_length = new_length
                        if new_length < min_distance:
                            min_distance = new_length
                        state_ray = [(ray[0][0], ray[0][1]), intersection]
            processed_rays.append(state_ray)
            ray_lengths.append(ray_length)
        return processed_rays, min_distance, ray_lengths


    def intersection(self, line1, line2, visibility):
        min_length = visibility
        intersection = []
        if not line1.intersection(line2).is_empty:
            ip = line1.intersection(line2)
            sp = list(line1.coords)[0]
            l = math.sqrt( ((sp[0]-ip.x)**2)+((sp[1]-ip.y)**2))
            if l < min_length:
                intersection = (ip.x, ip.y)
        else:
            return None
        return intersection

    def _create_scaler(self, scale, shapes):
        self.scaler = MinMaxScaler(feature_range=(0,scale))
        self.scaler.fit([list(p) for s in shapes for p in s])

    def _coordinates_to_pixels(self, shape):
        return self.scaler.transform(shape) 

    def _pixels_to_coordinates(self, shape):
        return self.scaler.inverse_transform(shape) 
