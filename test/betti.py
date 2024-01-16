import heron as he
import numpy as np


def main():
    adjacency_matrix = [
        [1, 4],
        [0, 2, 4],
        [1, 3],
        [2, 4, 5],
        [0, 1, 3],
        [3],
    ]
    print(he.betti_numbers(adjacency_matrix))


if __name__ == "__main__":
    main()
