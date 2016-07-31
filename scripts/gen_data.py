'''
This file is used to generate the data sets used throughout the website.
'''

import numpy as np
import matplotlib.pyplot as plt
import json


def gen_gaussian_clusters(points_per_cluster=200, min_x=0, max_x=500, min_y=0, max_y=500):
    x_range = max_x - min_x
    y_range = max_y - min_y

    std_dev = x_range * 0.05

    centers = [
                [x_range * 0.25, y_range * 0.25],
                [x_range * 0.75, y_range * 0.75]
              ]

    points = []
    for i in range(points_per_cluster):
        for c in centers:
            p_1 = np.random.normal(c[0], std_dev)
            p_2 = np.random.normal(c[1], std_dev)
            points.append([p_1, p_2])

    return points

def plot_scatter_test(data):
    data = np.array(data)

    plt.scatter(data[:,0], data[:,1])
    plt.show()

def save_data(file_name, data):
    file_path = '../static/data/' + file_name

    with open(file_path, 'w') as data_file:
        json.dump({'data': data}, data_file)


if __name__ == '__main__':
    gauss_clusters = gen_gaussian_clusters(points_per_cluster=100)
    save_data('gaussian.json', gauss_clusters)