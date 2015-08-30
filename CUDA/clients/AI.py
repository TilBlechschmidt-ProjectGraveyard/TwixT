__author__ = 'TheMegaTB'

from numba import cuda


@cuda.jit
def run(d_boards, d_actions, d_links, d_ais):
    KIND, VALUE, INPUTS, TARGETS = 0, 1, 2, 3
    ADD, SUB = 0, 1
    pos = cuda.grid(1)
    if pos < len(d_boards) and pos < len(d_actions) and pos < len(d_links) and pos < len(d_ais):
        board = d_boards[pos]
        links = d_links[pos]
        ais = d_ais[pos]

        # d_actions[pos] += 1
        # Set d_actions[pos] to the number of the field where the AI should place the dot

        # clear the inputs
        for line in ais[1:]:
            for node in line:
                node[INPUTS] = []

        # Set the inputs
        for i in range(len(board)):
            ais[0][i * 2][INPUTS] = board[i]
            ais[0][i * 2 + 1][INPUTS] = links[i]

        # Calculate everything line by line
        for line_number in range(len(ais)):
            for node in ais[line_number]:
                if node[KIND] == ADD:
                    node[VALUE] = 0
                    for input_value in node[INPUTS]:
                        node[VALUE] += input_value
                elif node[KIND] == SUB:
                    node[VALUE] = 0
                    if len(node[INPUTS]) > 0:
                        node[VALUE] = node[INPUTS][0]
                        for input_value in node[INPUTS][1:]:
                            node[VALUE] -= input_value

                for target in node[TARGETS]:
                    ais[line_number + 1][target][INPUTS].append(node[VALUE])

        d_actions[pos] = ais[-1][0][VALUE]
