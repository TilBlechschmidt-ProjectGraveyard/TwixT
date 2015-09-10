__author__ = 'Til Blechschmidt'

import sys

import server
from clients import AI, Enemy

round_counter = 0

def next_round():
    global round_counter

    if round_counter % 2 == 0:
        AI.run()
    else:
        Enemy.run()

    server.run()

    round_counter += 1

def main():
    if sys.argv[1]:
        rounds = sys.argv[1]
    else:
        round = 10



if __name__ == '__main__':
    main()