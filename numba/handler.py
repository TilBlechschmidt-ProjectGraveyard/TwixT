__author__ = ['Til Blechschmidt', 'Noah Peeters']

import sys
from timeit import default_timer as timer

import numpy as np

from numba import jit
import server
from clients import AI, Enemy


# ---- HELPER FUNCTIONS ----


@jit
def create_new_boards(count, size):
    return np.zeros((count, size))


@jit
def rotate_board_anti_clockwise(board, times=1):
    return np.append([], np.rot90(np.reshape(board, (24, 24)), times))


def rotate_board_clockwise(board, times=1):
    return rotate_board_clockwise(board, -times)


# ---- HELPER FUNCTIONS END ----

# ---- VARIABLE DECLARATIONS ----

round_counter = 0
boards = None
links = None

# ---- VARIABLE DECLARATIONS END ----


def reset(game_count, board_size):
    global boards, links
    b = create_new_boards(game_count, board_size)
    boards = b
    links = b


def next_round(gameid):
    global round_counter

    move = Enemy.run(boards[gameid])
    server.run(boards[gameid], links[gameid], move, 0)

    boards[gameid] = rotate_board_clockwise(boards[gameid])

    move = AI.run()
    server.run(boards[gameid], links[gameid], move, 1)

    boards[gameid] = rotate_board_anti_clockwise(boards[gameid])

    round_counter += 1


def main():
    if sys.argv[1]:
        rounds = int(sys.argv[1])
    else:
        rounds = 10

    reset(rounds + 1, 24*24)

    times = []
    for i in range(rounds):
        start = timer()
        next_round(1)
        times.append(timer() - start)

    for i in range(len(times)):
        print("Round " + str(i + 1) + ": ", times[i])


if __name__ == '__main__':
    main()
