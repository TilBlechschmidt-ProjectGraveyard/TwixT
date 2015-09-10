__author__ = ['Til Blechschmidt', 'Noah Peeters']

import numpy as np


def run(board, links, move, player):

    # Step 1: Look if the action is valid
    free_spaces = np.zeros(24 * 24)

    x = 0
    for i in range(len(board)):
        if i < 24 or i > (24 * 24 - 24):
            continue
        elif board[i] == 0:
            free_spaces[x] = i
            x += 1

    if move not in free_spaces:
        return False

    cons = [49, 47, 26, 22, -22, -26, -47, -49]

    for con in cons:
        if board[move+con] == player:
            print("NEW CONNECTION!")

    # Step 1.2: DO NOT CROSS THE BEAMS ahem LINES

    # Step 2: Check if somebody has won *yay*
