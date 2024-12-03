#include <string>
#include <regex>
#include <fstream>
#include <iostream>

int main()
{
    std::ifstream input_file("./input.txt");
    std::string file_contents((std::istreambuf_iterator<char>(input_file)),
                              (std::istreambuf_iterator<char>()));
    std::regex mul_regex(R"((don't\(\)|do\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)))",
                         std::regex_constants::ECMAScript);

    std::sregex_iterator regex_begin(file_contents.begin(), file_contents.end(), mul_regex);
    auto regex_end = std::sregex_iterator();

    uint64_t sum = 0;
    bool skip_mul_instructions = false;
    for (auto it = regex_begin; it != regex_end; ++it)
    {
        auto match_str = it->str();

        std::cout << match_str << std::endl;

        if (match_str == "don't()")
        {
            skip_mul_instructions = true;
        }
        else if (match_str == "do()")
        {
            skip_mul_instructions = false;
        }
        else
        {
            if (skip_mul_instructions)
                continue;

            auto first_number_start_idx = match_str.find('(') + 1;
            auto comma_idx = match_str.find(',', first_number_start_idx);
            auto closing_paren_idx = match_str.find(')', comma_idx);

            std::string lhs = match_str.substr(first_number_start_idx, comma_idx - first_number_start_idx);
            std::string rhs = match_str.substr(comma_idx + 1, closing_paren_idx - (comma_idx + 1));

            sum += std::stoi(lhs) * std::stoi(rhs);
        }
    }

    std::cout << sum << std::endl;

    return 0;
}