from matplotlib import pyplot as plt
from matplotlib import cm
import numpy as np
from tqdm import trange

from random import random, randint
from math import log10

NUM_BUCKETS = 7
dists = [[] for _ in range(NUM_BUCKETS)]

colors = cm.OrRd_r(np.linspace(0.2, 0.6, NUM_BUCKETS))

import signal
def sigint_handler(signal, frame):
    print('\n\n', dists)

    fig, axs = plt.subplots(1, NUM_BUCKETS, figsize=(NUM_BUCKETS*5, 6))

    base = 100
    for i in range(NUM_BUCKETS):
        base *= 10
        axs[i].hist(dists[i], color=colors[-i], label=f"10**{log10(base)}", orientation='horizontal', bins=30)
        axs[i].legend()
    fig.show()
    input('press enter to continue...')
signal.signal(signal.SIGINT, sigint_handler)

num_cycles = 0
while True:
    num_cycles += 1
    base = 100
    for i in range(NUM_BUCKETS):
        base *= 10
        mx = 0
        num = 0
        for iter in trange(2, int(base), desc=f"cycle #{num_cycles} bucket #{i}"):
            if random() < 0.5:
                num += 1
            if num/iter > mx:
                mx = num/iter
        dists[i].append(mx)
