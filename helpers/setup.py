import random as rnd
import warnings
from copy import deepcopy

import numpy as np
from numba import jit

from config import BOARD_WIDTH, FIELD_EMPTY, FIELD_SWAMP

__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']


class bcolors:
    BLUE = '\033[44m'
    SWAMP = '\033[42;32m'
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
            if board[x + x_off][y + y_off] != FIELD_EMPTY:
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
            board[swamp_loc[0] + x_off][swamp_loc[1] + y_off] = FIELD_SWAMP

    return board


@jit
def create_new_boards(count):
    # Initializing the board with zeros
    board = np.zeros((24, 24))

    # Generating swamps in the four corners
    board[0][0] = FIELD_SWAMP  # Top left corner
    board[23][0] = FIELD_SWAMP  # Top right corner
    board[0][23] = FIELD_SWAMP  # Bottom left corner
    board[23][23] = FIELD_SWAMP  # Bottom right corner

    # Generating swamps according to game rules
    board = generate_swamp(3, 3, board)
    board = generate_swamp(2, 2, board)
    board = generate_swamp(2, 2, board)
    board = generate_swamp(1, 1, board)

    # Multiplying the board with the amount of parallel games and returning it
    boards = []
    for i in range(count):
        boards.append(deepcopy(board))

    return boards


@jit
def create_new_links(count):
    # count = amount of games (multiplier for the board itself)
    # BOARD_WIDTH, BOARD_WIDTH = Dimensions of the board (2D Array)
    # 4 = 2 Coordinates (start and end point of link) each consisting of 2 values
    # return np.zeros((count, BOARD_WIDTH, BOARD_WIDTH, 4))
    links = []
    for i in range(count):
        links.append([])
    return links
