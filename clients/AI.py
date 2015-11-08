__author__ = ['Noah Peeters']

import random


class Node:
    def __init__(self):
        self.priorities = []

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
    def __init__(self, inputs, outputs):
        self.layers = []

    def add_layer(self, i):
        self.layers.insert(i, Layer())

    def add_nodes(self, i):
        self.layers[i].nodes.append(Node())
        for a in range(len(self.layers[i - 1])):
            self.layers[i].nodes[-1].add_priority()

    def calculate(self, values):
        self.calculate_layer(-1, values)

    def calculate_layer(self, i, values):
        if i == -len(self.layers):
            return self.layers[i].calculate(values)
        return self.layers[i].calculate(self.calculate_layer(i - 1, values))

    def mutate(self):
        pass


def run():
    pass
