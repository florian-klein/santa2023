#!/usr/bin/env python
import csv
import sys

import pandas as pd


def analyze_solution(solution_path, puzzles_path):
    csv.field_size_limit(sys.maxsize)
    puzzle_types = {}
    with open(puzzles_path, 'r') as f:
        reader = csv.reader(f)
        next(reader)
        for row in reader:
            puzzle_types[int(row[0])] = row[1]

    puzzles = []

    with open(solution_path, 'r') as f:
        reader = csv.reader(f)
        next(reader)
        for row in reader:
            id = int(row[0])
            score = len(row[1].split('.'))
            puzzle_type = puzzle_types[id]
            puzzles.append([id, puzzle_type, score])

    puzzles = pd.DataFrame(puzzles, columns=['id', 'puzzle_type', 'score'])
    puzzles.index = puzzles['id']
    puzzles = puzzles.drop('id', axis=1)

    print('Total score = {}'.format(puzzles['score'].sum()))
    print()
    print('Score by puzzle type:')
    print(puzzles.groupby('puzzle_type').sum().sort_values(by='score', ascending=False))
    print()
    print('Score by puzzle id (top 10):')
    print(puzzles.sort_values(by='score', ascending=False).head(10))


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print('Usage: python analyze.py <solution_path> <puzzles_path>')
        sys.exit(1)

    solution_path = sys.argv[1]
    puzzles_path = sys.argv[2]

    analyze_solution(solution_path, puzzles_path)


