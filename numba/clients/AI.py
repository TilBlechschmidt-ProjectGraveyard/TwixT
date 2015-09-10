__author__ = ['Til Blechschmidt', 'Noah Peeters']

import random


class Node:
    def __init__(self, priorities):
        self.priorities = priorities

    def calc(self, values):
        tmp = 0
        for i in range(len(values)):
            tmp += values[i] * self.priorities[i]
        return tmp

    def add_priority(self):
        self.priorities = random.random() * 2 - 1


class Layer:
    def __init__(self):
        self.nodes = []

    def calculate(self, values):
        return [node.calc(values) for node in self.nodes]

    def add_priority(self):
        for node in self.nodes:
            node.add_priority()

class AI:
    def __init__(self):
        self.layers = []

    def add_layer(self):
        pass

    def calculate(self):
        pass

    def mutate(self):
        pass


def run():
    pass
