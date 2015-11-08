from enum import Enum

BOARD_WIDTH = 24
BOARD_SIZE = BOARD_WIDTH * 2


class Field(Enum):
    empty = 0
    p1 = 1
    p2 = 2
    swamp = 3


SWAMP_BIG = [0, 1, 2, 24, 25, 26, 48, 49, 50]
SWAMP_SMALL = [0, 1, 24, 25]
SWAMP_TINY = [0]
