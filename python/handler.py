import sys
from timeit import default_timer as timer

import numpy as np
from numba import jit

import clients
import config
import helpers as h
import server

__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']


@jit
def reset(game_count):
    b = h.create_new_boards(game_count)
    l = h.create_new_links(game_count)
    s = np.zeros((game_count, 2))  # 2 = Amount of player's
    return b, l, s


def next_round(board, links, scores):
    move = clients.Enemy.run(board, 1)
    board, links, scores[0], won = server.run(board, links, scores[0], move, 1)

    move = clients.Enemy.run(board, 2)  # move = AI.run(board)
    board, links, scores[1], won = server.run(board, links, scores[1], move, 2)

    return board, links, scores, won


def main():
    parallel_games = 2
    rounds = 30
    if len(sys.argv) >= 2 and sys.argv[1]:
        rounds = int(sys.argv[1])

    boards, links, scores = reset(parallel_games)

    times = []
    finished_games = np.zeros(parallel_games, dtype=bool)
    for gid in range(parallel_games):
        for i in range(rounds):
            if not finished_games[gid]:
                start = timer()
                boards[gid], links[gid], scores[gid], finished_games[gid] \
                    = next_round(boards[gid], links[gid], scores[gid])
                times.append(timer() - start)
        if config.PRINT_BOARDS and parallel_games < config.PRINT_THRESHOLD:
            h.print_board(boards[gid])

    times = times[1:]  # Exclude the first round to prevent the time the JIT compilation takes to falsify the stats
    total_time = 0
    for i in range(len(times)):
        total_time += times[i]

    # ------------------------------------------------- STATS PRINTING -------------------------------------------------
    if config.PRINT_STATS:
        stats_header = "Simulated " + h.bold(str(parallel_games)) + " games with a total of " + h.bold(
            str(parallel_games * rounds)) + " rounds."

        spaces = " " * int((103 - len(stats_header)) / 1.5)
        stats_header = spaces + stats_header + spaces

        print('\n' * 2)

        print(stats_header)

        print("-------------------------------------------------------------------------------------------------------")

        print("Total time: " + '\033[1m' + str(total_time * 1000) + " milliseconds" + '\033[0m')
        print("Average time per round: " + '\033[1m' + str(np.mean(times) * 1000 * 1000) + " microseconds" + '\033[0m')
        print("-------------------------------------------------------------------------------------------------------")
        print("              (All times exclude the first round to compensate for the JIT compiler)")

        print('\n' * 2)

    # ----------------------------------------------- STATS PRINTING END -----------------------------------------------

if __name__ == '__main__':
    main()
