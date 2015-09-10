__author__ = 'TheMegaTB'

from numba import cuda


@cuda.jit
def run(d_boards, d_actions, d_links, d_ais):

    pos = cuda.grid(1)
    if pos < len(d_boards) and pos < len(d_actions) and pos < len(d_links) and pos < len(d_ais):
        board = d_boards[pos]
        links = d_links[pos]
        ais = d_ais[pos]

        d_actions[pos] += 1

        d_ais[pos] = 0
        # Set d_actions[pos] to the number of the field where the AI should place the dot
