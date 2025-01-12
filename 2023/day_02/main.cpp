#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <ostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

struct Set {
    int red;
    int blue;
    int green;

  public:
    friend std::ostream &operator<<(std::ostream &os, Set const &that) {
        return os << '{' << that.red << ", " << that.blue << ", " << that.green
                  << '}';
    }
};

std::vector<std::vector<Set>> parse_input(std::string const &path) {
    std::ifstream ifile(path);

    std::vector<std::vector<Set>> ret;

    for (std::string line; std::getline(ifile, line);) {
        line = line.substr(line.find(":") + 1);
        std::stringstream linestream(line);

        std::vector<Set> game;
        for (std::string chunk; std::getline(linestream, chunk, ';');) {
            std::stringstream chunkstream(chunk);

            Set set{0, 0, 0};
            for (std::string word; std::getline(chunkstream, word, ',');) {
                std::stringstream wordstream(word);

                int count;
                std::string color;
                if (wordstream >> count >> color) {
                    if (color == "red") {
                        set.red = count;
                    } else if (color == "blue") {
                        set.blue = count;
                    } else if (color == "green") {
                        set.green = count;
                    } else {
                        throw std::runtime_error("invalid color");
                    }
                } else {
                    throw std::runtime_error("invalid word");
                }
            }

            game.push_back(std::move(set));
        }

        ret.push_back(std::move(game));
    }

    return ret;
}

static const int MAX_RED = 12;
static const int MAX_GREEN = 13;
static const int MAX_BLUE = 14;

bool is_game_valid(std::vector<Set> const &game) {

    for (auto const &set : game) {
        if (set.red > MAX_RED || set.blue > MAX_BLUE || set.green > MAX_GREEN) {
            return false;
        }
    }

    return true;
}

int valid_game_id_sum(std::vector<std::vector<Set>> const &games) {
    int total = 0;
    int id = 1;
    for (auto const &game : games) {
        if (is_game_valid(game)) {
            total += id;
        }
        ++id;
    }

    return total;
}

Set colorwise_max_set(std::vector<Set> const &game) {
    assert(game.size() > 0);

    Set ret{0, 0, 0};

    for (auto const &set : game) {
        ret.red = std::max(ret.red, set.red);
        ret.green = std::max(ret.green, set.green);
        ret.blue = std::max(ret.blue, set.blue);
    }

    return ret;
}

int set_power_sum(std::vector<std::vector<Set>> const &games) {
    int total = 0;

    for (auto const &game : games) {
        Set max_set = colorwise_max_set(game);
        total += max_set.red * max_set.green * max_set.blue;
    }

    return total;
}

int main() {
    auto const games = parse_input("input/input.txt");

    std::cout << "part 1: " << valid_game_id_sum(games) << std::endl;
    std::cout << "part 2: " << set_power_sum(games) << std::endl;

    return 0;
}
