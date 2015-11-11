import helpers as h
from numba import jit


@jit
def run(board, links, move, player):
    # Step 1: Check if the move is valid
    if not h.move_is_valid(board, move):
        return [board, links]

    # Step 1.2.1: Check if a new connection is created
    links = h.check_link_possibility(links, board, move, player)

    # Step 2.1: Calculate the score

    # Step 2.2: Check if somebody has won *yay*

    # Step 3: Apply the move
    board[move] = player

    # Step 4: Return the board
    return board, links
