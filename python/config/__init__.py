from config import *
from numba import jit

import numpy as np

BOARD_SIZE = BOARD_WIDTH ** 2


@jit
def gen_swamp_rect(w, h):
    swamp = np.zeros(w * h)
    for j in range(h):
        for i in range(w):
            swamp[j * h + i] = j * BOARD_WIDTH + i
    return swamp


SWAMP_BIG = gen_swamp_rect(3, 3)
SWAMP_SMALL = gen_swamp_rect(2, 2)
SWAMP_TINY = gen_swamp_rect(1, 1)


@jit
def board_coord_to_index(x, y):
    return y * BOARD_WIDTH + x
