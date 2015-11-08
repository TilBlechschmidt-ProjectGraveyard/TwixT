__author__ = ['Til Blechschmidt', 'Merlin Brandt']

import numpy as np
from numba import jit

# Board structure:
# A one-dimensional 576 long array containing one of the following numbers:
# 0 = Free space
# 1 = Player 1 has blocked this space
# 2 = Player 2 has blocked this space
# 3 = This space is a swamp and therefore blocked too

BOARD_WIDTH = 24
BOARD_SIZE = BOARD_WIDTH ** 2

FIELD_EMPTY = 0
FIELD_P1 = 1
FIELD_P2 = 2
FIELD_SWAMP = 3


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


