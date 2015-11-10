__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']

import sys
from timeit import default_timer as timer

import numpy as np
from numba import jit

import helpers
import server
from clients import Enemy


@jit
def reset(game_count):
    b = helpers.create_new_boards(game_count)
    l = helpers.create_new_links(game_count)
    return b, l


def next_round(board_array, link_array):
    move = Enemy.run(board_array)
    board_array, link_array = server.run(board_array, link_array, move, 1)

    board_array = helpers.rotate_board_clockwise(board_array)

    # move = AI.run(board_array)
    move = Enemy.run(board_array)
    board_array, link_array = server.run(board_array, link_array, move, 2)

    board_array = helpers.rotate_board_anti_clockwise(board_array)

    return board_array, link_array


def main():
    parallel_games = 1
    rounds = 30
    if len(sys.argv) >= 2 and sys.argv[1]:
        rounds = int(sys.argv[1])

    boards, links = reset(parallel_games)

    times = []
    for game_id in range(parallel_games):
        for i in range(rounds):
            start = timer()
            boards[game_id], links[game_id] = next_round(boards[game_id], links[game_id])
            times.append(timer() - start)
        helpers.print_board(boards[game_id])

    total_time = 0
    for i in range(len(times)):
        # print("Round " + str(i + 1) + ": ", times[i] * 1000)
        total_time += times[i]

    print("Total time: " + '\033[1m' + str(total_time * 1000) + " milliseconds" + '\033[0m')
    print("Average time per round: " + '\033[1m' + str(np.mean(times[1:]) * 1000 * 1000) + " microseconds" + '\033[0m')


if __name__ == '__main__':
    main()
