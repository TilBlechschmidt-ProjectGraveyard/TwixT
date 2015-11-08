# -*- coding: latin-1 -*-

__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']

import math
import random
import sys
import warnings
from timeit import default_timer as timer

import numpy as np
from numba import jit

import server
from clients import Enemy
from config import BOARD_SIZE, BOARD_WIDTH, FIELD_EMPTY, FIELD_SWAMP, FIELD_P1, FIELD_P2, board_coord_to_index


# ---- HELPER FUNCTIONS ----


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
    random.seed()
    x = random.randrange(1, BOARD_WIDTH - width)
    random.seed()
    y = random.randrange(1, BOARD_WIDTH - height)
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


@jit
def rotate_board_anti_clockwise(board, times=1):
    return np.append([], np.rot90(np.reshape(board, (BOARD_WIDTH, BOARD_WIDTH)), times))


def rotate_board_clockwise(board, times=1):
    return rotate_board_anti_clockwise(board, -times)


def print_board(board):
    board_width = int(math.sqrt(len(board)))
    location = 0
    for row in range(board_width):
        print "|",
        for field in range(board_width):
            cur_loc = board[location]
            if (field == 0 or field == 23 or row == 0 or row == 23) and cur_loc == FIELD_EMPTY:
                if row == 0 or row == 23:
                    print bcolors.BASE_BLUE + '0' + bcolors.ENDC,
                else:
                    print bcolors.BASE_RED + '0' + bcolors.ENDC,
            elif cur_loc == FIELD_P1:
                print bcolors.RED + '1' + bcolors.ENDC,
            elif cur_loc == FIELD_P2:
                print bcolors.BLUE + '2' + bcolors.ENDC,
            elif cur_loc == FIELD_SWAMP:
                print bcolors.SWAMP + '3' + bcolors.ENDC,
            else:
                print "-",
            location += 1
        print "|"


# ---- HELPER FUNCTIONS END ----


@jit
def reset(game_count):
    b = create_new_boards(game_count)
    l = np.tile(np.zeros(BOARD_SIZE), (game_count, 1))
    return b, l


def next_round(board_array, link_array):
    move = Enemy.run(board_array)
    board_array, link_array = server.run(board_array, link_array, move, 1)

    board_array = rotate_board_clockwise(board_array)

    # move = AI.run(board_array)
    move = Enemy.run(board_array)
    board_array, link_array = server.run(board_array, link_array, move, 2)

    board_array = rotate_board_anti_clockwise(board_array)

    return board_array, link_array


def main():
    parallel_games = 1
    rounds = 1
    if len(sys.argv) >= 2 and sys.argv[1]:
        rounds = int(sys.argv[1])

    boards, links = reset(parallel_games)

    times = []
    for game_id in range(parallel_games):
        for i in range(rounds):
            start = timer()
            boards[game_id], links[game_id] = next_round(boards[game_id], links[game_id])
            times.append(timer() - start)
        print_board(boards[game_id])

    total_time = 0
    for i in range(len(times)):
        # print("Round " + str(i + 1) + ": ", times[i] * 1000)
        total_time += times[i]

    print("Total time: " + '\033[1m' + str(total_time * 1000) + " milliseconds" + '\033[0m')
    print("Average time per round: " + '\033[1m' + str(np.mean(times[1:]) * 1000 * 1000) + " microseconds" + '\033[0m')


if __name__ == '__main__':
    main()
