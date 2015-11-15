from numba import jit

from config import BOARD_SIZE, BOARD_WIDTH
from helpers import get_value

__author__ = ['Til Blechschmidt', 'Noah Peeters', 'Merlin Brandt']


@jit(nopython=True)
def value_in_array(value, array):
    result = False
    for i in array:
        if i == value:
            result = True
            break

    return result


@jit(nopython=True)
def is_inside_enemy_base(loc, player):  # TODO: Compress this function somehow (I'm sure it's possible)
    if player == 1 and (loc[0] == 0 or loc[0] == BOARD_WIDTH - 1):
        return True
    elif player == 2 and (loc[1] == 0 or loc[1] == BOARD_WIDTH - 1):
        return True
    return False


@jit(nopython=True)
def move_is_valid(board, move, player):
    # Player 1 = RED = Left to right
    # Player 2 = BLUE = Top to bottom
    field_empty = get_value(board, move) == 0
    inside_enemy_base = is_inside_enemy_base(move, player)

    return field_empty and not inside_enemy_base

"""
@jit(nopython=True)
def set_link(start, end, links):


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

    return links"""


@jit(nopython=True)
def calculate_score(links, board, player, score):
    pass


@jit#(nopython=True)
def check_link_possibility(links, board, loc, player, score):
    """
    # TODO: Make this fancier (if possible)
    link_set = False
    if board[loc - 2 - BOARD_WIDTH] == player:
        links = set_link(0, loc, links)
        link_set = True
    if board[loc - 1 - BOARD_WIDTH * 2] == player:
        links = set_link(1, loc, links)
        link_set = True
    if board[loc + 1 - BOARD_WIDTH * 2] == player:
        links = set_link(2, loc, links)
        link_set = True
    if board[loc + 2 - BOARD_WIDTH] == player:
        links = set_link(3, loc, links)
        link_set = True

    dest = loc + 2 + BOARD_WIDTH
    if dest < BOARD_SIZE and board[dest] == player:
        links = set_link(0, dest, links)
        link_set = True
    dest = loc + 1 + BOARD_WIDTH * 2
    if dest < BOARD_SIZE and board[dest] == player:
        links = set_link(1, dest, links)
        link_set = True
    dest = loc - 1 + BOARD_WIDTH * 2
    if dest < BOARD_SIZE and board[dest] == player:
        links = set_link(2, dest, links)
        link_set = True
    dest = loc - 2 + BOARD_WIDTH
    if dest < BOARD_SIZE and board[dest] == player:
        links = set_link(3, dest, links)
        link_set = True
"""
    link_set = False
    if board[loc[0] - 1][loc[1] + 2] == player:
        links.append([loc[0], loc[1], loc[0] - 1, loc[1] + 2])
        link_set = True
    if board[loc[0] + 1][loc[1] + 2] == player:
        links.append([loc[0], loc[1], loc[0] - 1, loc[1] + 2])
        link_set = True
    if board[loc[0] + 2][loc[1] + 1] == player:
        links.append([loc[0], loc[1], loc[0] - 1, loc[1] + 2])
        link_set = True
    if board[loc[0] + 2][loc[1] - 1] == player:
        links.append([loc[0], loc[1], loc[0] - 1, loc[1] + 2])
        link_set = True
    if board[loc[0] - 1][loc[1] - 2] == player:
        links.append([loc[0], loc[1], loc[0] - 1, loc[1] + 2])
        link_set = True
    if board[loc[0] + 1][loc[1] - 2] == player:
        links.append([loc[0], loc[1], loc[0] - 1, loc[1] + 2])
        link_set = True
    if board[loc[0] - 2][loc[1] - 1] == player:
        links.append([loc[0], loc[1], loc[0] - 1, loc[1] + 2])
        link_set = True
    if board[loc[0] - 2][loc[1] + 1] == player:
        links.append([loc[0], loc[1], loc[0] - 1, loc[1] + 2])
        link_set = True
    return links, link_set
