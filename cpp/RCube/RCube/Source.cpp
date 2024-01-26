#include "Cube.h"
#include "CubeViewer.h"
#include <fstream>
#include <iostream>
#include <sstream>

void StartNewCube() {
  unsigned int n = 0;    // Cube size
  unsigned int seed = 0; // Random seed
  int tmp = 0;

  printf("Starting a new cube\n");

  do {
    // Limit cube size from 1 to 65536
    printf("Cube Size (1-65536) : ");
    tmp = scanf("%u", &n);
  } while (n < 1 || n > 65536);

  // Create a new cube
  printf("Generating Cube...\n");
  Cube cube(n);

  // Scramble the cube using the seed value
  std::string move_seq = "r1.-f1";
  move_seq = invert_move_string(move_seq);
  printf("Scrambling Cube according to %s\n", move_seq.c_str());
  cube.scramble_according_to_move_sequence(move_seq);
  // cube.Scramble(seed);

  // No need to save progress for smaller cubes
  if (n >= 32768) {
    printf("Saving enabled\n");
    cube.SaveEnabled = true;
    cube.SaveCubeState();
  }

  // Print stats before solving
  cube.PrintStats();

  cube.MovesPerFrame = 0;

  cube.SaveEnabled = false;

  // Solve it!
  printf("Solving 3.0\n");
  std::string sol_str = cube.Solve();
  sol_str = sol_str.substr(0, sol_str.length() - 1);
  printf("Solution: %s\n", sol_str.c_str());

  // Print stats after solving
  cube.PrintStats();

  printf("Done\n");
  tmp = scanf("%i", &tmp);
}

void LoadExistingCube() {
  Cube cube;

  // Load the cube from the save state
  // cube.LoadCubeState();

  // Continue solving
  cube.Solve();

  cube.PrintStats();

  printf("Done\n");
  int tmp = scanf("%i", &tmp);
}

void ExampleImageOutput() {
  // Create a small cube
  Cube cube(3);

  // Scramble the cube with a random seed
  cube.scramble_according_to_move_sequence("r0");

  // Specify cube face, filename, image size, include gridlines
  CubeViewer::ExportFaceDiagram(cube.faces[0], "Front Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[1], "Right Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[2], "Back Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[3], "Left Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[4], "Top Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[5], "Bottom Face.png", 1024, true);
}

void visualize_cube(Cube cube) {
  CubeViewer::ExportFaceDiagram(cube.faces[0], "Front Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[1], "Right Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[2], "Back Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[3], "Left Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[4], "Top Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[5], "Bottom Face.png", 1024, true);
}

void visualize_cube_after_applying_move_string(Cube cube,
                                               std::string move_str) {
  cube.scramble_according_to_move_sequence(move_str);
  CubeViewer::ExportFaceDiagram(cube.faces[0], "Front Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[1], "Right Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[2], "Back Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[3], "Left Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[4], "Top Face.png", 1024, true);
  CubeViewer::ExportFaceDiagram(cube.faces[5], "Bottom Face.png", 1024, true);
}

void Omega() {

  Cube *cube = nullptr;

  std::ofstream out("kvalue1.csv", std::ios::app);

  for (int i = 0; i < 100; i++) {
    for (int n = 4; n <= 2048; n *= 2) {
      // create cube(n)
      cube = new Cube(n);

      cube->Scramble(i);

      cube->Solve();

      printf("%i : %.7f\n", n, cube->Hours * 3600.0);

      out << n << "," << cube->MoveCount << "," << (cube->Hours * 3600.0)
          << std::endl;

      delete cube;

      out.flush();
    }
  }

  out.close();
}

struct CubePuzzle {
  int cube_size;
};

struct Puzzle {
  int id;
  CubePuzzle cube_puzzle;
  std::string sample_solution;
};

Puzzle read_puzzle_from_csv(std::string solutions_path,
                            std::string puzzles_path, int puzzle_id) {
  // read csv from puzzles_path, find puzzle_id + 1th line, print this line
  // to stdout, and return the puzzle struct
  std::ifstream csv_file(puzzles_path);
  std::string line;
  int line_num = 0;
  Puzzle puzzle;
  while (std::getline(csv_file, line)) {
    if (line_num == puzzle_id + 1) {
      std::stringstream ss(line);
      std::string token;
      std::getline(ss, token, ',');
      puzzle.id = std::stoi(token);
      std::getline(ss, token, ',');
      // format cube_n/n/n
      std::stringstream ss2(token);
      std::string token2;
      std::getline(ss2, token2, '_');
      std::getline(ss2, token2, '_');
      puzzle.cube_puzzle.cube_size = std::stoi(token2);
      // print line
      // std::cout << line << std::endl;
      break;
    }
    line_num++;
  }
  csv_file.close();
  // open solutions file, look for puzzle_id + 1th line, find the second token
  // in the line and return it as the sample solution
  std::ifstream csv_file2(solutions_path);
  std::string line2;
  int line_num2 = 0;
  while (std::getline(csv_file2, line2)) {
    if (line_num2 == puzzle_id + 1) {
      std::stringstream ss(line2);
      std::string token;
      std::getline(ss, token, ',');
      std::getline(ss, token, ',');
      puzzle.sample_solution = token;
      // std::cout << line2 << std::endl;
      break;
    }
    line_num2++;
  }
  csv_file2.close();
  return puzzle;
}

int main(int argc, char **argv) {
  // parse id as argument
  int id = 0;
  if (argc > 1) {
    id = std::stoi(argv[1]);
  }

  // Uncomment to run the example image output
  Puzzle puzzle = read_puzzle_from_csv("../../../data/sample_submission.csv",
                                       "../../../data/puzzles.csv", id);
  // printf("Puzzle id: %i\n", puzzle.id);
  // printf("Cube size: %i\n", puzzle.cube_puzzle.cube_size);
  // printf("Sample solution: %s\n", puzzle.sample_solution.c_str());
  Cube cube(puzzle.cube_puzzle.cube_size);
  cube.Reset();
  // apply correctly in this order
  std::string sample_solution_string = puzzle.sample_solution;
  sample_solution_string = invert_move_string(sample_solution_string);
  cube.scramble_according_to_move_sequence(sample_solution_string);

  // above is correct, dont change it!!
  // below if up to discussion
  std::string sol_string = cube.Solve();
  sol_string = sol_string.substr(0, sol_string.length() - 1);
  // cube is not solved and at the initial state
  // cube.Reset();
  // cube.scramble_according_to_move_sequence(sample_solution_string);
  // cube.scramble_according_to_move_sequence(sol_string);
  // visualize_cube(cube);
  printf("%s\n", sol_string.c_str());

  return EXIT_SUCCESS;
}
