__author__ = 'TheMegaTB'

from numba import cuda
import numpy as np
import random
import numpy
import numbapro

@cuda.jit
def run(d_boards, d_links, d_actions):
    pos = cuda.grid(1)
    if pos < len(d_boards) and pos < len(d_actions) and pos < len(d_links):
        board = d_boards[pos]

        free_spaces = numpy.zeros(24*24-1)

        x = 0
        for i in range(len(board)):
            if i < 24 or i > (24*24-24):
                continue
            elif board[i] == 0:
                free_spaces[x] = i
                x += 1

        d_actions[pos] = free_spaces[random.randrange(len(free_spaces))]
        # Set d_actions[pos] to the number of the field where the AI should place the dot
