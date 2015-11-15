from numba import jit

import helpers as h


@jit
def run(board, links, score, move, player):
    won = False

    # Step 1: Check if the move is valid
    if not h.move_is_valid(board, move, player):
        return board, links, score, won

    # Step 1.2.1: Check if a new connection is created
    # links, score_outdated = h.check_link_possibility(links, board, move, player, score)

    # if score_outdated:
    #    # Step 2.1: Calculate the score
    #    score = h.calculate_score(links, board, player, score)
    #    # Step 2.2: Check if the player has won (*yay* or *nay*)
    #    won = (score >= 24)
    links.append([1, 2, 3, 4])

    # Step 3: Apply the move
    board = h.set_value(board, move, player)

    # Step 4: Return the board
    return board, links, score, won
