__author__ = 'TheMegaTB'

import sys
import re
from timeit import default_timer as timer

from numba import cuda
from numba import jit
import numpy as np

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


def setup(game_count, board_size, ai_str):

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
        AI.run[blockspergrid, threadsperblock](d_boards, d_actions, d_links, d_ais)
    else:
        Enemy.run[blockspergrid, threadsperblock](d_boards, d_actions, d_links)

    server.execute[blockspergrid, threadsperblock](d_boards, d_actions, d_links)
    d_actions = cuda.to_device(actions)

    round_counter += 1


def calculate_max_parallel_count():  # Available remaining buffer of about 20% VRAM is included in result
    available_mem = cuda.current_context().get_memory_info()[0]
    usable_mem = round(available_mem * 0.95)
    list_size = sys.getsizeof([])
    list_entry_size = sys.getsizeof([2]) - list_size
    return (usable_mem - list_size * 4) / (
    list_entry_size * 24 + list_entry_size + list_entry_size + list_entry_size * 24)


def main():
    # TODO: Split the available games into multiple evolution (say 10) lines with each a part of the available threads
    y = int(calculate_max_parallel_count())
    print("Running " + comma_me(str(y)) + " games in parallel.")
    start = timer()
    setup(y, 24, ['+-/'] * y)
    print("Setup: ", (timer() - start))

    start = timer()
    transfer_data()
    print("DTransfer: ", (timer() - start))
    start = timer()
    for i in range(1):
        next_round()
        print("Round " + str(i + 1) + ": ", (timer() - start))


if __name__ == '__main__':
    main()
