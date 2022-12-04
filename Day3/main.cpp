#include <iostream>
#include <fstream>
#include <string>
#include <string_view>
#include <algorithm>
#include <vector>
#include <numeric>

int get_priority(char letter) {
    //std::cout << letter << std::endl;

    if (letter >= 'a' && letter <= 'z') {
        return letter - 96;
    } else if (letter >= 'A' && letter <= 'Z') {
        return letter - 38;
    } else {
        return 1234;
    }
}


int main() {
    std::ifstream file("input.txt");
    if (file.is_open()) {
        std::string line;
        std::vector<int> end_priorities;
        while (std::getline(file, line)) {
            auto line_length = line.length();
            std::string_view view_line = line;

            auto first_part = view_line.substr(0, line_length / 2);
            auto second_part = view_line.substr(line_length / 2, line_length);

            std::for_each(first_part.begin(), first_part.end(), [&second_part, &end_priorities] (char c) {
                if ( std::ranges::find (second_part, c) != second_part.end()) {
                    char previous_letter;
                    if (c == previous_letter) {
                        // std::cout << "Duplicate: " << c << std::endl;
                    } else {
                        end_priorities.push_back(get_priority(c));
                        previous_letter = c;
                    }                }
            });
            // std::cout << "xddd: " << end_priorities.size() << std::endl;
        }
        file.close();

        std::cout << "Result: " << std::accumulate(end_priorities.begin(), end_priorities.end(), 0) << std::endl;
    }
    return 0;
}

