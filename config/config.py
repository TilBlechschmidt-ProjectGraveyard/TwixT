__author__ = ['Til Blechschmidt', 'Merlin Brandt']

# Board structure:
# A one-dimensional 576 long array containing one of the following numbers:
# 0 = Free space
# 1 = Player 1 has blocked this space
# 2 = Player 2 has blocked this space
# 3 = This space is a swamp and therefore blocked too

BOARD_WIDTH = 24
BOARD_SIZE = BOARD_WIDTH ** 2

FIELD_EMPTY = 0
FIELD_P1 = 1
FIELD_P2 = 2
FIELD_SWAMP = 3