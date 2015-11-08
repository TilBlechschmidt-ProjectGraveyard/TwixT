import numpy as np
from enum import Enum

BOARD_WIDTH = 24
BOARD_SIZE = BOARD_WIDTH ** 2


class Player(Enum):
    one = 1
    two = 2


class Field(Enum):
    empty = 0
    p1 = Player.one
    p2 = Player.two
    swamp = 3


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
    y * BOARD_WIDTH + x


