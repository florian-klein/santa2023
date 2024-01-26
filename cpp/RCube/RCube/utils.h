#include <string>
#include <vector>

struct MoveType {
  char move_name;
  int index;
  int step;
};

std::vector<std::string> split_string(std::string str, std::string delimiter);
std::string get_move_string(std::string move_name, int index, int step,
                            int row_size);
std::string join_string(std::vector<std::string> strings,
                        std::string delimiter);
inline MoveType parse_move_string_to_move(std::string move_str, int row_size);
std::vector<MoveType> parse_move_string_to_moves(std::string move_string,
                                                 int row_size);
std::string reverse_move_string(std::string move_string);
std::string invert_move_string(std::string move_string);
