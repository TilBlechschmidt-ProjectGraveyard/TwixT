__author__ = 'TheMegaTB'

import sys
import re
import random
import copy
from timeit import default_timer as timer

import numpy as np
from numba import cuda
from numba import jit

import server
from clients import AI, Enemy

boards = None
ai_strings = None
actions = None
links = None

parallel_count = 0

d_boards = None
d_ais = None
d_actions = None
d_links = None

round_counter = 0


# ---- HELPER FUNCTIONS ----

class Ai:
    @jit
    def __init__(self, structure, connections):
        self.basic_node = [0, 0, [], []]
        self.ai = []
        for s in structure:
            line = []
            for i in range(s):
                line.append(copy.deepcopy(self.basic_node))
                line[-1][0] = random.randint(0, 1)
            self.ai.append(line)  # kind, value, inputs, target

        for i in range(len(self.ai) - 1):
            for j in range(len(self.ai[i])):
                for c in range(connections):
                    d = random.choice(range(len(self.ai[i + 1])))
                    self.ai[i][j][3].append(d)


def comma_me(amount):
    orig = amount
    new = re.sub("^(-?\d+)(\d{3})", '\g<1>,\g<2>', amount)
    if orig == new:
        return new
    else:
        return comma_me(new)


@jit
def create_new_boards(count, size):
    return np.zeros((count, size))


@jit
def create_new_actions(count):
    return np.zeros(count)


# ---- HELPER FUNCTIONS END ----


def reset(game_count, board_size, ai_str):
    b = create_new_boards(game_count, board_size)
    act = create_new_actions(game_count)

    global boards, ai_strings, actions, links, parallel_count
    ai_strings = np.asarray(ai_str)
    actions = act
    boards = b
    links = b
    parallel_count = game_count


def transfer_data():
    global d_boards, d_actions, d_ais, d_links
    d_boards = cuda.to_device(boards)
    d_ais = cuda.to_device(ai_strings)
    d_actions = cuda.to_device(actions)
    d_links = cuda.to_device(links)


def next_round():
    global round_counter, d_actions, parallel_count

    threadsperblock = 32
    blockspergrid = (len(boards) + (threadsperblock - 1))

    if round_counter % 2 == 0:
        AI.run[blockspergrid, threadsperblock](d_boards, d_links, d_actions, d_ais)
    else:
        Enemy.run[blockspergrid, threadsperblock](d_boards, d_links, d_actions)

    server.execute[blockspergrid, threadsperblock](d_boards, d_links, d_actions)

    act = d_actions.copy_to_host()
    print(act)

    d_actions = cuda.to_device(actions)
    # cuda.synchronize()

    round_counter += 1


def calculate_max_parallel_count():  # Available remaining buffer of about 20% VRAM is included in result
    # TODO: MINOR - Take the amount of cores into account
    if not callable(getattr(cuda, "current_context", None)):
        return 1000
    else:
        available_mem = cuda.current_context().get_memory_info()[0]
        usable_mem = round(available_mem * 0.80)
        list_size = sys.getsizeof([])
        list_entry_size = sys.getsizeof([2]) - list_size
        return (usable_mem - list_size * 4) / (
            list_entry_size * 24 + list_entry_size + list_entry_size + list_entry_size * 24)


def main():
    # TODO: Split the available games into multiple evolution lines (say 10) with each a part of the available threads
    y = int(calculate_max_parallel_count())
    y = 1000
    print("Running " + comma_me(str(y)) + " games in parallel.")

    start = timer()
    reset(y, 24, [2, 1, -1, 4, 0.5, -0.5, 3, 1, -1, 4, 0.5, -0.5] * y)  # Ai([8, 2, 4, 3, 1], 4).ai
    print("Setup: ", (timer() - start))

    start = timer()
    transfer_data()
    print("DTransfer: ", (timer() - start))

    times = []
    for i in range(2):
        start = timer()
        next_round()
        times.append(timer() - start)

    for i in range(len(times)):
        print("Round " + str(i + 1) + ": ", times[i])

    res_ais = d_ais.copy_to_host()
    print(res_ais)


if __name__ == '__main__':
    main()
