from matplotlib import pyplot as plt
from matplotlib import cm
import numpy as np
from tqdm import tqdm

from itertools import count
from random import random, randint
from math import log10

NUM_BUCKETS = 50
NTILES = 8
dists = [[] for _ in range(NUM_BUCKETS)]

colors = cm.OrRd_r(np.linspace(0.1, 0.8, NTILES+1))

def plot():
    # print('\n\n', dists)

    for i, d in tqdm(enumerate(dists), desc="sorting..."):
        d.sort()

    print('plotting...')
    for ntile in range(NTILES):
        # print('len of step', len(dists[2]), 'ntile', int(ntile/NTILES*len(dists[2])))
        plt.plot(range(2, len(dists)), [step[int(ntile/NTILES*len(step))] for step in dists[2:]], label=f"{ntile}/{NTILES}", color=colors[ntile])

    plt.plot(range(2, len(dists)), [step[-1] for step in dists[2:]], label="max", color=colors[-1])
    plt.legend()

    # histagrams
    # fig, axs = plt.subplots(1, NUM_BUCKETS, figsize=(NUM_BUCKETS*5, 6))
    # base = 100
    # for i in range(NUM_BUCKETS):
    #     base *= 10
    #     axs[i].hist(dists[i], color=colors[-i], label=f"10**{log10(base)}", orientation='horizontal', bins=30)
    #     axs[i].legend()
    plt.show()

import signal
def sigint_handler(signal, frame):
    print('got signal... sorting...')
    plot()
    input('press enter to continue...')
signal.signal(signal.SIGINT, sigint_handler)

# num_cycles = 0
# for num_cycles in tqdm(count()):
for num_cycles in tqdm(range(1000000)):
    # num_cycles += 1
    mx = 0
    num = 0
    for iter in range(2, NUM_BUCKETS):
        if random() < 0.5:
            # print(mx, num, iter)
            num += 1
        if num/iter > mx:
            mx = num/iter
        # dists[iter].append(max(mx, 0.5))
        dists[iter].append(mx)
        # dists[iter].append(num/iter)

plot()
