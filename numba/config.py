from enum import Enum
import numpy as np

BOARD_WIDTH = 24
BOARD_SIZE = BOARD_WIDTH ** 2

class Field(Enum):
    empty = 0
    p1 = 1
    p2 = 2
    swamp = 3




SWAMP_BIG = gen_swamp_rect(3, 3)
SWAMP_SMALL = gen_swamp_rect(2, 2)
SWAMP_TINY = gen_swamp_rect(1, 1)

def gen_swamp_rect(w, h):
    swamp = np.zeros(w*h)
    for j in range(h):
        for i in range(w):
            swamp[j * h + i] = j * BOARD_WIDTH + i
    return swamp


def board_coord_to_index(x, y):
    y * BOARD_WIDTH + x


