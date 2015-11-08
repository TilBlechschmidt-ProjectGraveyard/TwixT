__author__ = ['Til Blechschmidt', 'Noah Peeters']

import random as rnd

import numpy as np
from numba import jit

from config import BOARD_SIZE, BOARD_WIDTH, Field


@jit(nopython=True)
def run(board):
    free_spaces = np.zeros(BOARD_SIZE)
    x = 0

    for i in range(BOARD_WIDTH - 1, BOARD_SIZE - BOARD_WIDTH):
        if board[i] == Field.empty:
            free_spaces[x] = i
            x += 1

    free_spaces = free_spaces[:x]

    return free_spaces[rnd.randrange(len(free_spaces))]
