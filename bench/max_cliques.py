import numpy as np
import networkx as nx
import time

import heron as he

# NETWORKX
def networkx_cliques(A):
    start = time.time()
    G = nx.Graph(A)
    count = 0
    for _ in nx.find_cliques(G):
        count += 1
    elapsed = time.time() - start
    return elapsed, count


# HERON
def heron_cliques(A):
    adjacency_matrix = [np.nonzero(row)[0].tolist() for row in A]
    return he.find_cliques(adjacency_matrix)



def main():
    # CREATING THE RANDOM MATRIX
    n = 1500
    matrix = np.random.uniform(0, 1, size=(n, n))
    matrix = (matrix + matrix.T) / 2
    np.fill_diagonal(matrix, 0)

    # TRESHOLDING
    threshold = 0.25
    A = np.zeros_like(matrix)
    A[matrix < threshold] = 1
    np.fill_diagonal(A, 0)

    elapsed, count = networkx_cliques(A)
    print("NETWORKX")
    print(f"elapsed: {elapsed}, count: {count}")
    elapsed, count = heron_cliques(A)
    print("HERON")
    print(f"elapsed: {elapsed}, count: {count}")
    elapsed, count = heron_better_cliques(A)
    print("TOMITA")
    print(f"elapsed: {elapsed}, count: {count}")


if __name__ == "__main__":
    main()
