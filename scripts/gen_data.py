'''
This file is used to generate the data sets used throughout the website.
'''

import numpy as np
import matplotlib.pyplot as plt
import json


def gen_gaussian_clusters(points_per_cluster=100, min_x=0, max_x=500, min_y=0, max_y=500):
    x_range = max_x - min_x
    y_range = max_y - min_y

    std_dev = x_range * 0.05

    centers = [
                [min_x + x_range * 0.25, min_y + y_range * 0.25],
                [min_x + x_range * 0.75, min_y + y_range * 0.75]
              ]

    points = []
    for i in range(points_per_cluster):
        for c in centers:
            p_1 = np.random.normal(c[0], std_dev)
            p_2 = np.random.normal(c[1], std_dev)
            points.append([p_1, p_2])

    return points


def gen_concentric_circle(points_per_cluster=100, min_x=0, max_x=500, min_y=0, max_y=500):
    x_range = max_x - min_x
    y_range = max_y - min_y

    assert(x_range == y_range)

    grid_center = [min_x + x_range * 0.5, min_y + y_range * 0.5]

    # Total diameter of 0.2 * range in center
    mid_radius = x_range * 0.1;

    # Diameter of outer circle is 0.6
    outer_radius = x_range * 0.3

    points = []
    for p in range(points_per_cluster):
        inner_r = np.random.uniform(0, mid_radius)
        inner_theta = np.random.uniform(0, 2 * np.pi)

        points.append([grid_center[0] + inner_r * np.cos(inner_theta),
                        grid_center[1] + inner_r * np.sin(inner_theta)])

        outer_theta = np.random.uniform(0, 2 * np.pi)

        points.append([grid_center[0] + outer_radius * np.cos(outer_theta),
                        grid_center[1] + outer_radius * np.sin(outer_theta)])

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
    # gauss_clusters = gen_gaussian_clusters(points_per_cluster=100)
    # save_data('gaussian.json', gauss_clusters)

    circle_clusters = gen_concentric_circle(points_per_cluster=150)
    save_data('circles.json', circle_clusters)
