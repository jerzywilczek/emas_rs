import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import matplotlib as mpl
from os.path import exists
import time
import sys

mpl.use('tkagg')

plt.style.use('fivethirtyeight')
figure, axis = plt.subplots(2, 2)
figure.set_size_inches(9.5, 6.5)
figure.set_dpi(100)


def animate(i, path):
    data = pd.read_csv(path)
    xs = data['timestamp']

    axis[0, 0].clear()
    axis[0, 0].plot(xs, data['average energy'], linewidth=1)
    axis[0, 0].set_title("average energy")

    axis[0, 1].clear()
    axis[0, 1].plot(xs, data['agents amount'], linewidth=1)
    axis[0, 1].set_title('agents amount')

    axis[1, 0].clear()
    axis[1, 0].plot(xs, data['average fitness'], linewidth=1)
    axis[1, 0].set_title('average fitness')

    axis[1, 1].clear()
    axis[1, 1].plot(xs, data['best living'], linewidth=1)
    axis[1, 1].set_yscale('log')
    axis[1, 1].set_title('best living')

    plt.tight_layout()


"""
args: [filename]
"""
if __name__ == '__main__':
    _, filename = sys.argv
    file_path = './{}'.format(filename)
    while not exists(file_path):
        time.sleep(0.5)

    ani = FuncAnimation(plt.gcf(), lambda i: animate(i, file_path), interval=500)

    plt.tight_layout()
    plt.show()
