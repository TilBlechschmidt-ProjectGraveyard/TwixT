__author__ = ['Til Blechschmidt', 'Noah Peeters']

import numpy as np
import random as rnd


def run(board):
    free_spaces = np.empty(24 * 24)

    x = 0
    for i in range(len(board)):
        if i < 24 or i > (24 * 24 - 24):
            continue
        elif board[i] == 0:
            free_spaces[x] = i
            x += 1

    return free_spaces[rnd.randrange(len(free_spaces))]
