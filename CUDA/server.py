__author__ = 'TheMegaTB'

from numba import cuda


@cuda.jit
def execute(d_boards, d_actions):
    pos = cuda.grid(1)
    if pos < len(d_boards) and pos < len(d_actions):
        d_board = cuda.device_array(d_boards[pos])
        d_action = cuda.device_array(d_actions[pos])
