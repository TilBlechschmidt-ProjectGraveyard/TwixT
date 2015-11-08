__author__ = ['Til Blechschmidt', 'Merlin Brandt']

import numpy as np

BOARD_WIDTH = 24
BOARD_SIZE = BOARD_WIDTH ** 2

FIELD_EMPTY = 0
FIELD_P1 = 1
FIELD_P2 = 2
FIELD_SWAMP = 3


def gen_swamp_rect(w, h):
    swamp = np.zeros(w * h)
    for j in range(h):
        for i in range(w):
            swamp[j * h + i] = j * BOARD_WIDTH + i
    return swamp

SWAMP_BIG = gen_swamp_rect(3, 3)
SWAMP_SMALL = gen_swamp_rect(2, 2)
SWAMP_TINY = gen_swamp_rect(1, 1)


def board_coord_to_index(x, y):
    return y * BOARD_WIDTH + x


