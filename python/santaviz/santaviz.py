#!/usr/bin/env python3
import pandas as pd
from dash import Dash, html, dcc
import argparse
import plotly.express as px
import dash_bootstrap_components as dbc


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

    # Create the app with DARKLY theme for a dark gray layout
    app = Dash(__name__, external_stylesheets=[dbc.themes.DARKLY])
    app.title = "Santaviz 2023"

    # Create the layout
    app.layout = dbc.Container(
        [
            html.H1("Santaviz 2023", style={'color': 'white'}),
            dbc.Row(
                [
                    dbc.Col(
                        [
                            dcc.Graph(
                                figure=px.bar(
                                    solution,
                                    x="id",
                                    y="score",
                                    hover_data=["puzzle_type"],
                                    labels={"id": "Puzzle ID", "score": "Score"},
                                    title="Score per puzzle",
                                ).update_layout(
                                    {
                                        'plot_bgcolor': 'rgba(50, 50, 50, 1)',
                                        'paper_bgcolor': 'rgba(50, 50, 50, 1)',
                                        'font': {'color': 'white'}
                                    }
                                )
                            ),
                        ],
                        md=6,
                    ),
                    dbc.Col(
                        [
                            dcc.Graph(
                                figure=px.bar(
                                    solution,
                                    x=solution["puzzle_type"].unique(),
                                    y=solution.groupby("puzzle_type")["score"].sum().sort_values(ascending=False),
                                    labels={"x": "Puzzle type", "y": "Score"},
                                    title="Score per puzzle type",
                                ).update_layout(
                                    {
                                        'plot_bgcolor': 'rgba(50, 50, 50, 1)',
                                        'paper_bgcolor': 'rgba(50, 50, 50, 1)',
                                        'font': {'color': 'white'}
                                    }
                                )
                            ),
                        ],
                        md=6,
                    ),
                ],
                align='center'
            ),
        ],
        fluid=True,
    )

    # Run the app
    app.run_server(debug=True, port=args.port)

