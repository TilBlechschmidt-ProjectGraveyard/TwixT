__author__ = 'TheMegaTB'

from numba import cuda


@cuda.jit
def execute(d_boards, d_actions, d_links):
    pos = cuda.grid(1)
    if pos < len(d_boards) and pos < len(d_actions) and pos < len(d_links):
        d_board = d_boards[pos]
        d_action = d_actions[pos]

        # Step 1: Look if the action is valid

        # Step 1.1: Is there already a pin?

        # Step 1.2: DO NOT CROSS THE BEAMS ahem LINES

        # Step 2: Check if somebody has won *yay*
