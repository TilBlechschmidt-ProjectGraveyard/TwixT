__author__ = 'TheMegaTB'

from numba import cuda


@cuda.jit
def run(d_boards, d_actions, d_links):
    pos = cuda.grid(1)
    if pos < len(d_boards) and pos < len(d_actions) and pos < len(d_links):
        board = d_boards[pos]

        d_actions[pos] += 1
        # Set d_actions[pos] to the number of the field where the AI should place the dot
