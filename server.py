__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']

from numba import jit

from config import BOARD_SIZE, BOARD_WIDTH


# from boardlocation import BoardLocation


@jit(nopython=True)
def value_in_array(value, array):
    result = False
    for i in array:
        if i == value:
            result = True
            break

    return result


@jit
def move_is_valid(board, move):
    return BOARD_WIDTH <= move <= BOARD_SIZE - BOARD_WIDTH and board[move] == 0


@jit
def run(board, links, move, player):
    # Step 1: Check if the move is valid
    if not move_is_valid(board, move):
        # print("INVALID MOVE")
        return [board, links]

    # Step 1.2.1: Check if a new connection is created
    cons = [49, 47, 26, 22, -22, -26, -47, -49]

    for con in cons:
        if move + con < BOARD_SIZE:
            if board[move + con] == player:
                # Step 1.2.2: DO NOT CROSS THE BEAMS ahem LINES
                # Step 1.2.3: Set the connection
                # Step 2.1: Calculate the score
                pass

    # Step 2.2: Check if somebody has won *yay*

    # Step 3: Apply the move
    board[move] = player

    # Step 4: Return the board
    return board, links
