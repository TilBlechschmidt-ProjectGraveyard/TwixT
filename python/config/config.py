__author__ = ['Til Blechschmidt', 'Merlin Brandt', 'Noah Peeters', 'Max Zager']

# Here you can define how many games with how many rounds should be run
PARALLEL_GAMES = 1
ROUNDS_PER_GAME = 30

# The server assumes that the board is always rectangular so 'width = height' and 'total size = width**2'
BOARD_WIDTH = 24

# The constants for the board defining what state gets what ID
# (basically these don't matter but you can touch them anyway)
FIELD_EMPTY = 0
FIELD_P1 = 1
FIELD_P2 = 2
FIELD_SWAMP = 3

# Whether to print boards and stats or not and the threshold (amount of games) where it should stop printing
PRINT_STATS = False
PRINT_BOARDS = True
PRINT_THRESHOLD = 100
