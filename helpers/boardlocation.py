from config import BOARD_WIDTH

# Wrapper around a board location that can either store an index or components
class BoardLocation(object):

    @property
    def index(self):
        if self._ind is None:
            self._ind = self._xy[1] * BOARD_WIDTH + self._xy[0]
        return self._ind

    @index.setter
    def index(self, ind):
        self._ind = ind
        self._xy = None

    @property
    def xy(self):
        if self._xy is None:
            self._xy = self._ind % BOARD_WIDTH, self._ind / BOARD_WIDTH
        return self._xy

    @xy.setter
    def xy(self, xy):
        self._ind = None
        self._xy = xy


    @property
    def x(self):
        return self.xy[0]

    @property
    def y(self):
        return self.xy[1]

    def __init__(self, a):
        self._ind = None
        self._xy = None

        if type(a) is tuple:
            self._xy = a
        else:
            self._ind = a