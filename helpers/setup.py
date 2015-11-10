__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']

import random as rnd
import warnings

import numpy as np
from numba import jit

from config import BOARD_SIZE, BOARD_WIDTH, FIELD_EMPTY, FIELD_SWAMP, board_coord_to_index


class bcolors:
    BLUE = '\033[44m'
    SWAMP = '\033[42m'
    WHITE = '\033[47m'
    RED = '\033[41m'
    BASE_BLUE = '\033[34;47m'
    BASE_RED = '\033[31;47m'
    ENDC = '\033[0m'


@jit
def generate_swamp_location(width, height):
    rnd.seed()
    x = rnd.randrange(1, BOARD_WIDTH - width)
    rnd.seed()
    y = rnd.randrange(1, BOARD_WIDTH - height)
    return x, y


# whether in the specified rect in the board, all fields are field
@jit
def is_rect_empty(board, x, y, width, height):
    for x_off in range(width):
        for y_off in range(height):
            cur_ind = board_coord_to_index(x + x_off, y + y_off)

            if board[cur_ind] != FIELD_EMPTY:
                return False
    return True


@jit
def generate_swamp(width, height, board):
    swamp_loc = 0, 0
    counter = 0

    # Repeat this simulation until a valid location is found
    while True:
        swamp_loc = generate_swamp_location(width, height)

        if is_rect_empty(board, swamp_loc[0], swamp_loc[1], width, height):
            break

        counter += 1

        if counter == 10:
            warnings.warn("Swamp generation takes too long")

        if counter == 50:
            warnings.warn("Couldn't generate swamp.")
            return board

    # Generate the swamp
    for x_off in range(width):
        for y_off in range(height):
            board[board_coord_to_index(swamp_loc[0] + x_off, swamp_loc[1] + y_off)] = FIELD_SWAMP

    return board


@jit
def create_new_boards(count):
    # Initializing the board with zeros
    board = np.repeat(FIELD_EMPTY, BOARD_SIZE)

    # Generating swamps according to game rules
    board = generate_swamp(3, 3, board)
    board = generate_swamp(2, 2, board)
    board = generate_swamp(2, 2, board)
    board = generate_swamp(1, 1, board)

    # Generating swamps in the four corners
    board[0] = FIELD_SWAMP  # Top left corner
    board[BOARD_WIDTH - 1] = FIELD_SWAMP  # Top right corner
    board[BOARD_SIZE - BOARD_WIDTH] = FIELD_SWAMP  # Bottom left corner
    board[BOARD_SIZE - 1] = FIELD_SWAMP  # Bottom right corner

    # Multiplying the board with the amount of parallel games and returning it
    return np.tile(board, (count, 1))
