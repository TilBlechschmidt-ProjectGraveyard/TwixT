__author__ = ['Til Blechschmidt', 'Noah Peeters']

import random as rnd

import numpy as np
from numba import jit


@jit(nopython=True)
def run(board):
    free_spaces = np.zeros(528)
    x = 0

    for i in range(len(board)):
        # Check for enemy 'base'
        if i < 24 or i > (24 * 24 - 24):
            continue
        elif board[i] == 0:
            free_spaces[x] = i
            x += 1

    free_spaces = free_spaces[:x]

    return free_spaces[rnd.randrange(len(free_spaces))]
