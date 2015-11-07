__author__ = ['Til Blechschmidt', 'Noah Peeters']

import sys, random, math
from timeit import default_timer as timer

import numpy as np

from numba import jit
import server
from clients import AI, Enemy


# ---- HELPER FUNCTIONS ----

SWAMP_BIG = [0, 1, 2, 22, 23, 24, 46, 47, 48]
SWAMP_SMALL = [0, 1, 23, 24]
SWAMP_TINY = [0]


@jit
def generate_swamp_location(size):
    random.seed()
    return random.randrange(0, size, 1)


@jit
def generate_swamp(size_constant, board):
    board_size = len(board)
    valid = False
    swamp_loc = 0
    while not valid:
        swamp_loc = generate_swamp_location(board_size)
        invalid = False
        for i in size_constant:
            # print(i, swamp_loc + i)
            if (swamp_loc + i) >= board_size:
                # print("OUT OF BOUNDS")
                invalid = True
                break
            if board[swamp_loc + i] == 3:
                # print("OVERLAPPING")
                invalid = True
                break
        if not invalid:
            valid = True

    for i in size_constant:
        board[swamp_loc + i] = 3

    return board


@jit
def create_new_boards(count, board_size):
    board_width = math.sqrt(board_size)

    # Initializing the board with zeros
    board = np.zeros(board_size)

    # Generating swamps according to game rules
    board = generate_swamp(SWAMP_BIG, board)
    board = generate_swamp(SWAMP_SMALL, board)
    board = generate_swamp(SWAMP_SMALL, board)
    board = generate_swamp(SWAMP_TINY, board)

    # Generating swamps in the four corners
    board[0] = 3                            # Top left corner
    board[board_width-1] = 3                # Top right corner
    board[board_size-1 - board_width-1] = 3   # Bottom left corner
    board[board_size-1] = 3                   # Bottom right corner

    # Multiplying the board with the amount of parallel games and returning it
    return np.tile(board, (count, 1))


@jit
def rotate_board_anti_clockwise(board, times=1):
    return np.append([], np.rot90(np.reshape(board, (24, 24)), times))


def rotate_board_clockwise(board, times=1):
    return rotate_board_anti_clockwise(board, -times)


# ---- HELPER FUNCTIONS END ----

# ---- VARIABLE DECLARATIONS ----

round_counter = 0
boards = None
links = None

# ---- VARIABLE DECLARATIONS END ----


def reset(game_count, board_size):
    global boards, links
    b = create_new_boards(game_count, board_size)
    print b[0]
    boards = b
    links = b


def next_round(gameid):
    global round_counter

    move = Enemy.run(boards[gameid])
    server.run(boards[gameid], links[gameid], move, 1)

    boards[gameid] = rotate_board_clockwise(boards[gameid])

    # move = AI.run(boards[gameid])
    Enemy.run(boards[gameid])
    server.run(boards[gameid], links[gameid], move, 2)

    boards[gameid] = rotate_board_anti_clockwise(boards[gameid])

    round_counter += 1


def main():
    rounds = 1
    if len(sys.argv) >= 2 and sys.argv[1]:
        rounds = int(sys.argv[1])

    reset(rounds + 1, 24*24)

    times = []
    for i in range(rounds):
        start = timer()
        next_round(1)
        times.append(timer() - start)

    total_time = 0
    for i in range(len(times)):
        print("Round " + str(i + 1) + ": ", times[i]*1000)
        total_time += times[i]

    print("Total time: ", total_time*1000)


if __name__ == '__main__':
    main()
