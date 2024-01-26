#include "utils.h"
#include <cassert>
#include <string>

std::vector<std::string> split_string(std::string str, std::string delimiter) {
  std::vector<std::string> strings;
  size_t pos = 0;
  std::string token;
  while ((pos = str.find(delimiter)) != std::string::npos) {
    token = str.substr(0, pos);
    strings.push_back(token);
    str.erase(0, pos + delimiter.length());
  }
  strings.push_back(str);
  return strings;
}

std::string join_string(std::vector<std::string> strings,
                        std::string delimiter) {
  std::string result = "";
  for (int i = 0; i < strings.size(); i++) {
    result += strings[i];
    if (i != strings.size() - 1) {
      result += delimiter;
    }
  }
  return result;
}

std::string get_move_string(std::string move_name, int index, int step,
                            int row_size) {
  std::string kaggle_move_string = (step < 0) ? "-" + move_name : move_name;
  kaggle_move_string += std::to_string(index);
  std::string result_str = "";
  for (int i = 0; i < abs(step); i++) {
    result_str += kaggle_move_string + ".";
  }
  return result_str;
}

inline MoveType parse_move_string_to_move(std::string move_str, int row_size) {
  // has format "r3" or "-r10" or "f", thus {- -> make index
  // neg}{move_name}{index}
  int step = 1;
  if (move_str[0] == '-') {
    step = -1;
    move_str = move_str.substr(1);
  }
  // std::cout << "(parse_move_str) move_str: " << move_str << std::endl;
  std::string move_name = move_str.substr(0, 1);
  // std::cout << "(parse_move_str) move_name: " << move_name << std::endl;
  int index = std::stoi(move_str.substr(1));
  // std::cout << "(parse_move_str) index: " << index << std::endl;
  char move_name_char = move_name[0];
  index = row_size - index - 1;
  MoveType result = {move_name_char, index, step};
  return result;
}

std::vector<MoveType> parse_move_string_to_moves(std::string move_string,
                                                 int row_size) {
  std::vector<std::string> move_strings = split_string(move_string, ".");
  std::vector<MoveType> moves;
  for (int i = 0; i < move_strings.size(); i++) {
    // std::cout << "move_strings[i]: " << move_strings[i] << std::endl;
    // std::cout << "parse_move_string_to_move(move_strings[i]): "
    //           << parse_move_string_to_move(move_strings[i]).move_name
    //           << std::endl;
    moves.push_back(parse_move_string_to_move(move_strings[i], row_size));
  }
  return moves;
}

std::string reverse_move_string(std::string move_string) {
  // split the string by ".", reverse the substrings and join them with "."
  std::vector<std::string> move_strings = split_string(move_string, ".");
  std::vector<std::string> reversed_move_strings;
  for (int i = 0; i < move_strings.size(); i++) {
    std::string move_str = move_strings[i];
    reversed_move_strings.push_back(move_str);
  }
  std::reverse(reversed_move_strings.begin(), reversed_move_strings.end());
  std::string result = join_string(reversed_move_strings, ".");
  return result;
}

std::string invert_move_string(std::string move_string) {
  // reverses the string an every move without at "-" gets a "-" and every move
  // with a "-" gets removed
  std::vector<std::string> move_strings = split_string(move_string, ".");
  std::vector<std::string> inverted_move_strings;
  for (int i = 0; i < move_strings.size(); i++) {
    std::string move_str = move_strings[i];
    if (move_str[0] == '-') {
      move_str = move_str.substr(1);
    } else {
      move_str = "-" + move_str;
    }
    inverted_move_strings.push_back(move_str);
  }
  std::reverse(inverted_move_strings.begin(), inverted_move_strings.end());
  std::string result = join_string(inverted_move_strings, ".");
  return result;
}

void test_case_0() {
  std::string input = "1 2 3 4 5 6 7 8 9";
  std::string delimiter = " ";
  std::vector<std::string> expected = {"1", "2", "3", "4", "5",
                                       "6", "7", "8", "9"};
  std::vector<std::string> actual = split_string(input, delimiter);
  assert(0 == 1);
  assert(actual == expected);
}

void test_case_split_dot() {
  std::string input = "f1.f2.f3.f4.f5.f6.f7.f8.f9";
  std::string delimiter = ".";
  std::vector<std::string> expected = {"f1", "f2", "f3", "f4", "f5",
                                       "f6", "f7", "f8", "f9"};
  std::vector<std::string> actual = split_string(input, delimiter);
  assert(actual == expected);
}
