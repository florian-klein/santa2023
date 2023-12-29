#!/usr/bin/env python3
import pandas as pd
from dash import Dash, html, dcc
import argparse


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="A visualization dashboard for the Santa2023 puzzles in solutions")
    parser.add_argument("--port", type=int, default=8050, help="Port to run the server on.")
    parser.add_argument("--solution", type=str, default="./../santa_utils/submission.csv", help="Path to the solution file.")
    parser.add_argument("--puzzleinfo", type=str, default="./../../data/puzzle_info.csv", help="Path to the puzzle info file.")
    parser.add_argument("--puzzles", type=str, default="./../../data/puzzles.csv", help="Path to the puzzles file.")
    args = parser.parse_args()

    # Load the data
    solution = pd.read_csv(args.solution)
    puzzle_info = pd.read_csv(args.puzzleinfo)
    puzzles = pd.read_csv(args.puzzles)

    # Compute the solution scores
    solution["score"] = solution["moves"].apply(lambda x: len(x.split(".")))

    # Append the puzzle type to the solution (solution["id"] is a key for puzzles["id"], puzzles["puzzle_type"] is a key for puzzle_info["puzzle_type"])
    solution = solution.merge(puzzles, on="id")
    solution = solution.merge(puzzle_info, on="puzzle_type")
    solution = solution.sort_values(by="score", ascending=False)

    # Create the app
    app = Dash(__name__)
    app.title = "Santaviz 2023"

    # We want a nice plot for the score distribution of the solutions and a ranking of the puzzles by score (with the puzzle type as hue)
    # Then we want a plot for the score distribution of the puzzle types

    # Create the layout
    app.layout = html.Div(
        [
            html.H1("Santaviz 2023"),
            html.H2("Solution scores"),
            dcc.Graph(
                figure={
                    "data": [
                        {
                            "x": solution["id"],
                            "y": solution["score"],
                            "type": "bar",
                            "text": solution["puzzle_type"],
                            "name": "Score per puzzle",
                            "marker": {"color": solution["puzzle_type"]},
                            "textposition": "auto",
                        },
                    ],
                    "layout": {
                        "title": "Score per puzzle",
                        "xaxis": {"title": "Puzzle ID"},
                        "yaxis": {"title": "Score"},
                    },
                }
            ),
            dcc.Graph(
                figure={
                    "data": [
                        {
                            "y": solution.groupby("puzzle_type")["score"].sum().sort_values(ascending=False),
                            "x": solution["puzzle_type"].unique(),
                            "type": "bar",
                            "name": "Score per puzzle type",
                        },
                    ],
                    "layout": {
                        "title": "Score per puzzle type",
                        "xaxis": {"title": "Puzzle type"},
                        "yaxis": {"title": "Score"},
                    },
                }
            ),
        ]
    )

    # Run the app
    app.run_server(debug=True, port=args.port)

