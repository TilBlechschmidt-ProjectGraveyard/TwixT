from numba import jit
from config import BOARD_SIZE, BOARD_WIDTH

__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']


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


@jit(nopython=True)
def set_link(direction, loc, links):
    # print("Setting link at ", loc, " into direction ", direction)
    # TODO: Make this fancier (if possible)
    links[loc][direction] = 10  # 10 = Here's a link
    if direction == 0:
        links[loc + BOARD_WIDTH][1] += 1
        links[loc - 1 + BOARD_WIDTH][2] += 1
        links[loc - 2 + BOARD_WIDTH][2] += 1
        links[loc - 1][3] += 1
        links[loc - 1][2] += 1
        links[loc - 1][1] += 1
        links[loc - 2][3] += 1
        links[loc - 2][2] += 1
        links[loc - 3][3] += 1

    if direction == 1:
        links[loc - 1 + BOARD_WIDTH][2] += 1
        links[loc + 1][0] += 1
        links[loc - 1][3] += 1
        links[loc - 1][2] += 1
        links[loc - 2][3] += 1
        links[loc - BOARD_WIDTH][0] += 1
        links[loc - 1 - BOARD_WIDTH][3] += 1
        links[loc - 1 - BOARD_WIDTH][2] += 1
        links[loc - 2 - BOARD_WIDTH][3] += 1

    if direction == 2:
        links[loc + 1 + BOARD_WIDTH][1] += 1
        links[loc - 1][3] += 1
        links[loc + 1][0] += 1
        links[loc + 1][1] += 1
        links[loc + 2][0] += 1
        links[loc - BOARD_WIDTH][3] += 1
        links[loc + 1 - BOARD_WIDTH][0] += 1
        links[loc + 1 - BOARD_WIDTH][1] += 1
        links[loc + 2 - BOARD_WIDTH][0] += 1

    if direction == 3:
        links[loc + BOARD_WIDTH][2] += 1
        links[loc + 1 + BOARD_WIDTH][1] += 1
        links[loc + 2 + BOARD_WIDTH][1] += 1
        links[loc + 1][0] += 1
        links[loc + 1][1] += 1
        links[loc + 1][2] += 1
        links[loc + 2][0] += 1
        links[loc + 2][1] += 1
        links[loc + 3][0] += 1

    return links


@jit(nopython=True)
def check_link_possibility(links, board, loc, player):
    # TODO: Make this fancier (if possible)
    if board[loc - 2 - BOARD_WIDTH] == player:
        links = set_link(0, loc, links)
    if board[loc - 1 - BOARD_WIDTH * 2] == player:
        links = set_link(1, loc, links)
    if board[loc + 1 - BOARD_WIDTH * 2] == player:
        links = set_link(2, loc, links)
    if board[loc + 2 - BOARD_WIDTH] == player:
        links = set_link(3, loc, links)

    dest = loc + 2 + BOARD_WIDTH
    if dest < BOARD_SIZE and board[dest] == player:
        links = set_link(0, dest, links)
    dest = loc + 1 + BOARD_WIDTH * 2
    if dest < BOARD_SIZE and board[dest] == player:
        links = set_link(1, dest, links)
    dest = loc - 1 + BOARD_WIDTH * 2
    if dest < BOARD_SIZE and board[dest] == player:
        links = set_link(2, dest, links)
    dest = loc - 2 + BOARD_WIDTH
    if dest < BOARD_SIZE and board[dest] == player:
        links = set_link(3, dest, links)

    return links


@jit
def run(board, links, move, player):
    # Step 1: Check if the move is valid
    if not move_is_valid(board, move):
        return [board, links]

    # Step 1.2.1: Check if a new connection is created
    links = check_link_possibility(links, board, move, player)

    # Step 2.1: Calculate the score

    # Step 2.2: Check if somebody has won *yay*

    # Step 3: Apply the move
    board[move] = player

    # Step 4: Return the board
    return board, links
