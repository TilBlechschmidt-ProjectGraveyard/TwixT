__author__ = 'themegatb'

import tk


def vtoxy(v):
    return (v - (v % 24)) / 24, v % 24


def get_player_color(player):
    return ['white', 'red', 'blue'][player]


class drawing():
    def __init__(self):
        master = tk.Tk()

        self.canvas_width = 300
        self.canvas_height = 300
        self.w = tk.Canvas(master, width=self.canvas_width, height=self.canvas_height)
        self.w.pack()

    def draw(self, board, links):
        for i in range(24 * 24):
            x, y = vtoxy(i)
            self.w.create_circle(x * 10 + 10, y * 10 + 10, 5, fill=get_player_color(board[i]))
