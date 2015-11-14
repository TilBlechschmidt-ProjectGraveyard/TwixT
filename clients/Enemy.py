import random as rnd

import numpy as np
from numba import jit

from config import BOARD_SIZE, BOARD_WIDTH, FIELD_EMPTY
from helpers import is_inside_enemy_base

__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']


@jit(nopython=True)
def run(board, player):
    free_spaces = np.zeros((BOARD_SIZE, 2), dtype=np.uint8)
    amount = 0

    for x in range(BOARD_WIDTH):
        for y in range(BOARD_WIDTH):
            if board[x][y] == FIELD_EMPTY and not is_inside_enemy_base((x, y), player):
                free_spaces[amount][0] = x
                free_spaces[amount][1] = y
                amount += 1

    return free_spaces[rnd.randrange(amount)]
