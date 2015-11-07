__author__ = ['Til Blechschmidt', 'Noah Peeters']

import numpy as np
from numba import jit


@jit(nopython=True)
def value_in_array(value, array):
    result = False
    for i in array:
        if i == value:
            result = True
            break

    return result


@jit(nopython=True)
def move_is_valid(board, move):
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

    if not value_in_array(move, free_spaces):
        # The move is invalid
        return False
    else:
        # The move is valid
        return True


@jit
def run(board, links, move, player):
    # Board structure:
    # A one-dimensional 576 long array containing one of the following numbers:
    # 0 = Free space
    # 1 = Player 1 has blocked this space
    # 2 = Player 2 has blocked this space
    # 3 = This space is a swamp and therefore blocked too

    # Step 1: Check if the move is valid
    if not move_is_valid(board, move):
        # print("INVALID MOVE")
        return [board, links]

    # Step 1.2.1: Check if a new connection is created
    cons = [49, 47, 26, 22, -22, -26, -47, -49]

    for con in cons:
        if move + con < 24 * 24:
            if board[move + con] == player:
                pass  # print("NEW CONNECTION!")

    # Step 1.2.2: DO NOT CROSS THE BEAMS ahem LINES

    # Step 1.2.3: Set the connection

    # Step 2: Check if somebody has won *yay*

    # Step 3: Apply the move
    board[move] = player

    # Step 4: Return the board
    return [board, links]
