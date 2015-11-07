__author__ = ['Til Blechschmidt', 'Noah Peeters']

import numpy as np


def run(board, links, move, player):

    # Board structure:
        # 1D 576 (24^2) long array containing one of the following numbers:
        # 0 = Free space
        # 1 = Player 1 has blocked this space
        # 2 = Player 2 has blocked this space
        # 3 = This space is a swamp and therefore blocked too

    # Step 1: Check if the move is valid
    free_spaces = np.empty(24 * 24)

    x = 0
    for i in range(len(board)):
        if i < 24 or i > (24 * 24 - 24):
            continue
        elif board[i] == 0:
            free_spaces[x] = i
            x += 1

    if move not in free_spaces:
        return False

    # Step 1.2: Check if a new connection is created
    cons = [49, 47, 26, 22, -22, -26, -47, -49]

    for con in cons:
        if move+con < 24*24:
            if board[move+con] == player:
                print("NEW CONNECTION!")

    # Step 2: DO NOT CROSS THE BEAMS ahem LINES

    # Step 2: Check if somebody has won *yay*
