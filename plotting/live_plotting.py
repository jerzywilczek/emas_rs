import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import matplotlib as mpl
mpl.use('tkagg')

plt.style.use('fivethirtyeight')
figure, axis = plt.subplots(2, 2)
figure.set_size_inches(9.5, 6.5)
figure.set_dpi(100)

def animate(i):
    data = pd.read_csv('./outputs.csv')
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
    axis[1, 1].set_title('best living')

    plt.tight_layout()


ani = FuncAnimation(plt.gcf(), animate, interval=500)

plt.tight_layout()
plt.show()