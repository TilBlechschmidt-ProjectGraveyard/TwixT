__author__ = 'TheMegaTB'

import sys
from timeit import default_timer as timer

from numba import cuda
from numba import jit
import numpy as np

from clients import AI

boards = None
ai_strings = None
actions = None

d_boards = None
d_ais = None
d_actions = None


@jit
def create_new_boards(count, size):
    return np.zeros((count, size))


@jit
def create_new_actions(count):
    return np.zeros(count)


def setup(game_count, board_size, ai_str):
    b = create_new_boards(game_count, board_size)
    act = create_new_actions(game_count)

    global boards, ai_strings, actions
    ai_strings = np.asarray(ai_str)
    actions = act
    boards = b


def transfer_data():
    global d_boards, d_actions, d_ais
    d_boards = cuda.to_device(boards)
    d_ais = cuda.to_device(ai_strings)
    d_actions = cuda.to_device(actions)


def start_round():
    threadsperblock = 32
    blockspergrid = (len(boards) + (threadsperblock - 1))
    AI.run[blockspergrid, threadsperblock](d_boards, d_ais, d_actions)


def calculate_max_parallel_count():  # Available remaining buffer of about 20% VRAM is included in result
    available_mem = cuda.current_context().get_memory_info()[0]
    usable_mem = round(available_mem * 0.95)
    list_size = sys.getsizeof([])
    list_entry_size = sys.getsizeof([2]) - list_size
    return (usable_mem - list_size * 3) / (list_entry_size * 24 + list_entry_size + list_entry_size)


def main():
    y = int(calculate_max_parallel_count())
    start = timer()
    setup(y, 24, ['+-/'] * y)
    print("Setup: ", (timer() - start))

    start = timer()
    transfer_data()
    print("DTransfer: ", (timer() - start))
    start = timer()
    start_round()
    print("Calculation: ", (timer() - start))
    x = d_actions.copy_to_host()
    print(x)


if __name__ == '__main__':
    main()
