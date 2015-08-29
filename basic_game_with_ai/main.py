__author__ = 'Noah'

import random, copy

operations = [ADD, SUB] = [0, 1]
KIND, VALUE, INPUTS, TARGETS = 0, 1, 2, 3

class Ai:
    def __init__(self, structure, connections):
        self.basic_node = [random.choice(operations), 0, [], []]
        self.ai = []
        for s in structure:
            line = []
            for i in range(s):
                line.append(copy.deepcopy(self.basic_node))
            self.ai.append(line)  # kind, value, inputs, target

        for i in range(len(self.ai)-1):
            for j in range(len(self.ai[i])):
                for c in range(connections):
                    d = random.choice(range(len(self.ai[i+1])))
                    self.ai[i][j][TARGETS].append(d)

    def simple_calc(self, inputs):
        return self.calc([[a] for a in inputs])

    def calc(self, inputs):
        # Clear all inputs
        for b in self.ai:
            for a in b:
                a[INPUTS] = []

        # Set the inputs
        for i in range(len(inputs)):
            self.ai[0][i][INPUTS] = inputs[i]

        # Calculate everything line by line
        for i in range(len(self.ai)):
            for a in self.ai[i]:
                if a[KIND] == ADD:
                    a[VALUE] = 0
                    for c in a[INPUTS]:
                        a[VALUE] += c
                elif a[KIND] == SUB:
                    a[VALUE] = 0
                    if len(a[INPUTS]) > 0:
                        a[VALUE] = a[INPUTS][0]
                        for c in a[INPUTS][1:]:
                            a[VALUE] -= c
                elif a[KIND] == MUL:
                    a[VALUE] = 0
                    if len(a[INPUTS]) > 0:
                        a[VALUE] = a[INPUTS][0]
                        for c in a[INPUTS][1:]:
                            a[VALUE] /= c
                for b in a[TARGETS]:
                    self.ai[i+1][b][INPUTS].append(a[VALUE])

        return [a[VALUE] for a in self.ai[-1]]

    def get_ai(self):
        return copy.deepcopy(self.ai)

    def set_ai(self, ai):
        self.ai = ai

    def mutate(self):
        mode = random.randint(0, 1)
        if mode == 0:  # change operator
            random.choice(random.choice(self.ai))[0] = random.choice(operations)
        elif mode == 1:  # add node
            line = random.randint(1, len(self.ai)-2)
            random.choice(self.ai[line]).append(copy.deepcopy(self.basic_node))
            number = len(self.ai[line])-1
            random.choice(self.ai[line-1])[TARGETS].append(number)
            random.choice(self.ai[line-1])[TARGETS].append(number)
            self.ai[line][number][TARGETS].append(random.choice(range(len(self.ai[line+1]))))
            self.ai[line][number][TARGETS].append(random.choice(range(len(self.ai[line+1]))))



ai = Ai([6, 5, 1, 1, 1, 1, 1], 4)

old_food_count = -1
old_ai = []
max_food = -1
generation = 0
while True:
    player_x = player_y = 0
    food_x = random.randint(0, 10)
    food_y = random.randint(0, 10)
    food_count = 0
    for testlap in range(10):
        for lap in range(1000):
            step = round(ai.simple_calc([food_x-player_x, food_y-player_y, food_x, food_y, player_x, player_y])[0]/5.0) % 4
            #print(player_y)
            #print(ai.simple_calc([food_x-player_x, food_y-player_y])[0])
            #print(step)
            if step == 0:
                player_x += 1
            elif step == 1:
                player_y += 1
            elif step == 2:
                player_x -= 1
            else:
                player_y -= 1
            if player_x == food_x and player_y == food_y:
                food_count += 1
                food_x = random.randint(0, 10)
                food_y = random.randint(0, 10)

    if (food_count/10) >= old_food_count:
        old_food_count == (food_count/10)
        ai.mutate()
        old_ai = ai.get_ai()
        if (food_count/10) >= max_food:
            max_food = (food_count/10)
            print('Generation ' + str(generation) + ': ' + str((food_count/10)))
    else:
        ai.set_ai(old_ai)
        ai.mutate()
    generation += 1

print(max_food)

