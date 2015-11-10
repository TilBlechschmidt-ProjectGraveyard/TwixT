__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']

from numba import jit
from config import BOARD_SIZE, BOARD_WIDTH, board_coord_to_index


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
def set_link(direction, location, board, links):
    links[location][direction] = 10  # 10 = Here's a link
    x = 0
    y = 0
    bcti = board_coord_to_index
    if direction == 0:
        links[bcti(x, y + 1)][1] += 1  # links[x    ][y + 1].bridge[1] += 1
        links[bcti(x - 1, y + 1)][2] += 1  # links[x - 1][y + 1].bridge[2] += 1
        links[bcti(x - 2, y + 1)][2] += 1  # links[x - 2][y + 1].bridge[2] += 1
        links[bcti(x - 1, y)][3] += 1  # links[x - 1][y    ].bridge[3] += 1
        links[bcti(x - 1, y)][2] += 1  # links[x - 1][y    ].bridge[2] += 1
        links[bcti(x - 1, y)][1] += 1  # links[x - 1][y    ].bridge[1] += 1
        links[bcti(x - 2, y)][3] += 1  # links[x - 2][y    ].bridge[3] += 1
        links[bcti(x - 2, y)][2] += 1  # links[x - 2][y    ].bridge[2] += 1
        links[bcti(x - 3, y)][3] += 1  # links[x - 3][y    ].bridge[3] += 1

    if direction == 1:
        links[x - 1][y + 1].bridge[2] += 1
        links[x + 1][y].bridge[0] += 1
        links[x - 1][y].bridge[3] += 1
        links[x - 1][y].bridge[2] += 1
        links[x - 2][y].bridge[3] += 1
        links[x][y - 1].bridge[0] += 1
        links[x - 1][y - 1].bridge[3] += 1
        links[x - 1][y - 1].bridge[2] += 1
        links[x - 2][y - 1].bridge[3] += 1

    if direction == 2:
        links[x + 1][y + 1].bridge[1] += 1
        links[x - 1][y].bridge[3] += 1
        links[x + 1][y].bridge[0] += 1
        links[x + 1][y].bridge[1] += 1
        links[x + 2][y].bridge[0] += 1
        links[x][y - 1].bridge[3] += 1
        links[x + 1][y - 1].bridge[0] += 1
        links[x + 1][y - 1].bridge[1] += 1
        links[x + 2][y - 1].bridge[0] += 1

    if direction == 3:
        links[x][y + 1].bridge[2] += 1
        links[x + 1][y + 1].bridge[1] += 1
        links[x + 2][y + 1].bridge[1] += 1
        links[x + 1][y].bridge[0] += 1
        links[x + 1][y].bridge[1] += 1
        links[x + 1][y].bridge[2] += 1
        links[x + 2][y].bridge[0] += 1
        links[x + 2][y].bridge[1] += 1
        links[x + 3][y].bridge[0] += 1
         


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
