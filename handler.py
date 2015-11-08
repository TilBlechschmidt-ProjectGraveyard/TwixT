# -*- coding: latin-1 -*-

__author__ = ['Til Blechschmidt', 'Noah Peeters']

import math
import random
import sys
from timeit import default_timer as timer

import numpy as np
from numba import jit

import config as cfg
import server
from clients import Enemy
from config import BOARD_SIZE, BOARD_WIDTH, Field, Player, board_coord_to_index

# ---- VARIABLE DECLARATIONS ----


round_counter = 0
boards = None
links = None


# ---- VARIABLE DECLARATIONS END ----

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
    x = random.randrange(1, BOARD_WIDTH - width + 1)
    y = random.randrange(1, BOARD_WIDTH - height + 1)
    return x, y

def is_rect_free(board, x, y, width, height):
    pass

@jit
def generate_swamp(width, height, board):
    swamp_loc = 0, 0

    # Repeat this simulation until a valid location is found
    while True:
        swamp_loc = generate_swamp_location()
        # whether this swamp touches another
        touches = True
        # go through every cell of this swamp
        for x_off in range(width):
            for y_off in range(height):
                cur_loc = swamp_loc[0] + x_off, swamp_loc[0] + y_off
                cur_ind = board_coord_to_index(cur_loc[0], cur_loc[1])

                if board[cur_ind] == Field.swamp:
                    touches = True
                    break

    # Generate the swamp
    for i in swamp_model:
        board[swamp_loc + i] = Field.swamp

    return board


@jit
def create_new_boards(count):

    # Initializing the board with zeros
    board = np.repeat(Field.empty, BOARD_SIZE)

    # Generating swamps according to game rules
    print("TEST1")
    board = generate_swamp(cfg.SWAMP_BIG, board)
    board = generate_swamp(cfg.SWAMP_SMALL, board)
    board = generate_swamp(cfg.SWAMP_SMALL, board)
    board = generate_swamp(cfg.SWAMP_TINY, board)
    print("TEST2")
    # Generating swamps in the four corners
    board[0] = Field.swamp  # Top left corner
    board[BOARD_WIDTH - 1] = Field.swamp  # Top right corner
    board[BOARD_SIZE - BOARD_WIDTH] = Field.swamp  # Bottom left corner
    board[BOARD_SIZE - 1] = Field.swamp  # Bottom right corner

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
            cur_loc = int(board[location])
            if (field == 0 or field == 23 or row == 0 or row == 23) and cur_loc == Field.empty:
                if row == 0 or row == 23:
                    print bcolors.BASE_BLUE + str(cur_loc) + bcolors.ENDC,
                else:
                    print bcolors.BASE_RED + str(cur_loc) + bcolors.ENDC,
            elif cur_loc == Field.p1:
                print bcolors.RED + str(cur_loc) + bcolors.ENDC,
            elif cur_loc == Field.p2:
                print bcolors.BLUE + str(cur_loc) + bcolors.ENDC,
            elif cur_loc == Field.swamp:
                print bcolors.SWAMP + str(cur_loc) + bcolors.ENDC,
            elif cur_loc == 4:
                print bcolors.WHITE + str(cur_loc) + bcolors.ENDC,
            else:
                print "-",
            location += 1
        print "|"


# ---- HELPER FUNCTIONS END ----


def reset(game_count):
    global boards, links
    b = create_new_boards(game_count)
    boards = b
    links = b


def next_round(game_id):
    global round_counter

    move = Enemy.run(boards[game_id])
    boards[game_id], links[game_id] = server.run(boards[game_id], links[game_id], move, Player.one)

    boards[game_id] = rotate_board_clockwise(boards[game_id])

    # move = AI.run(boards[gameid])
    move = Enemy.run(boards[game_id])
    boards[game_id], links[game_id] = server.run(boards[game_id], links[game_id], move, Player.two)

    boards[game_id] = rotate_board_anti_clockwise(boards[game_id])
    round_counter += 1


def main():
    rounds = 30
    if len(sys.argv) >= 2 and sys.argv[1]:
        rounds = int(sys.argv[1])

    reset(rounds + 1)

    times = []
    for i in range(rounds):
        start = timer()
        next_round(1)
        times.append(timer() - start)

    print_board(boards[1])

    total_time = 0
    for i in range(len(times)):
        # print("Round " + str(i + 1) + ": ", times[i] * 1000)
        total_time += times[i]

    print("Total time: " + '\033[1m' + str(total_time * 1000) + " milliseconds" + '\033[0m')
    print("Average time per round: " + '\033[1m' + str(np.mean(times[1:]) * 1000 * 1000) + " microseconds" + '\033[0m')


if __name__ == '__main__':
    main()
