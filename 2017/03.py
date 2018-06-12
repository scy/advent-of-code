from itertools import islice
import math
from sys import argv

x = int(argv[1])

sqrt = math.sqrt(x)
width = math.ceil(sqrt)
if width % 2 == 0:
    width += 1
ring = math.floor(width / 2)

min = int(math.pow(2 * (ring - 1) + 1, 2)) + 1
max = int(math.pow(2 *  ring      + 1, 2))

steps = x - min
to_edge_center = abs((ring - 1) - (steps % (2 * ring)))
to_center = to_edge_center + ring

print(to_center)
