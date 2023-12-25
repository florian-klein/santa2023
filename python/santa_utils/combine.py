#!/usr/bin/env python
import csv
import os
import sys


def combine_solutions(solutions_dir, output_file):
    """Combine solutions from a directory into a single CSV file.

    Args:
        solutions_dir (str): Path to directory containing solutions.
        output_file (str): Path to output file.
    """
    csv.field_size_limit(sys.maxsize)

    # Get all files in directory.
    files = os.listdir(solutions_dir)

    # Get all CSV files.
    csv_files = [file for file in files if file.endswith('.csv')]

    # Find the best solution for each id
    best_solutions = {}
    for file in csv_files:
        with open(os.path.join(solutions_dir, file), 'r') as f:
            reader = csv.reader(f)
            next(reader)
            for row in reader:
                id = int(row[0])
                moves = row[1]
                score = len(moves.split('.'))
                if (id not in best_solutions) or (score < len(best_solutions[id].split('.'))):
                    best_solutions[id] = moves


    score = 0
    # Write the best solutions to output file.
    with open(output_file, 'w') as f:
        writer = csv.writer(f)
        writer.writerow(['id', 'moves'])
        for id, moves in best_solutions.items():
            writer.writerow([id, moves])
            score += len(moves.split('.'))
    return score


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print('Usage: python combine.py <solutions_dir> <output_file>')
        sys.exit(1)

    solutions_dir = sys.argv[1]
    output_file = sys.argv[2]

    score = combine_solutions(solutions_dir, output_file)
    print('Final score = {}'.format(score))
