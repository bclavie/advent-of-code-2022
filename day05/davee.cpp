#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <tuple>
#include <vector>
#include <deque>

struct Op {
    unsigned int quantity;
    unsigned int from;
    unsigned int to;
};

using stack_list = std::vector<std::deque<char>>;
using op_list = std::vector<Op>;

static void read_stack_line(stack_list &stack, std::string_view line) {
    for (auto i = 0; i < stack.size(); ++i) {
        // each column is 4 character wide, so we can directly
        // reference the index
        const auto index = 4*i;

        // index points to the column which is either 3 spaces "   " or
        // of the form "[X]" where X is a character. grab the character
        // and skip if it is a space
        const auto ch = line[index+1];

        if (ch == ' ') {
            continue;
        }

        stack[i].push_back(ch);
    }
}

static stack_list parse_stack_layout(const std::string& layout) {
    auto lines = std::vector<std::string>{};
    auto ss = std::stringstream{layout};

    for (std::string line; std::getline(ss, line, '\n');) {
        lines.push_back(line);
    }

    // remove the last line (its just a guide for the number of buckets)
    lines.pop_back();

    // we could calculate the number of stack buckets based on the 
    // total length of the of the line provided. each bucket input
    // is 3 bytes, and each separator is 1 byte. If n is the number of
    // buckets then 3n + (n - 1) is the length of the line for a given n.
    // so we can calculate n based on line length l by: n = (l + 1)/4
    const auto num_buckets = (lines.front().length() + 1) / 4;

    // we construct a list of stacks to represent our data
    stack_list stacks(num_buckets);

    for (auto& line : lines) {
        read_stack_line(stacks, line);
    }

    return stacks;
}

static Op parse_op(const std::string& line) {
    // each line is of the form "move X from Y to Z"
    unsigned int quantity, from, to = 0;
    sscanf(line.c_str(), "move %u from %u to %u", &quantity, &from, &to);
    return { quantity, from-1, to-1};
}

static op_list parse_op_list(const std::string& ops) {
    auto op_list = std::vector<Op>{};
    auto ss = std::stringstream{ops};

    for (std::string line; std::getline(ss, line, '\n');) {
        op_list.push_back(parse_op(line));
    }

    return op_list;
}

static std::tuple<stack_list, op_list> split(const std::string& input) {
    const auto pos = input.find("\n\n");
    auto layout = std::string(input, 0, pos);
    auto ops = std::string(input, pos+2);
    return {parse_stack_layout(layout), parse_op_list(ops)};
}

static std::tuple<stack_list, op_list> read_input(void) {
    std::ifstream file("input.txt");
    std::stringstream buffer;

    if (!file.is_open()) {
        std::cerr << "couldnt open file" << std::endl;
        std::abort();
    }

    buffer << file.rdbuf();
    return split(buffer.str());
}

static void part1(stack_list stack_list, op_list op_list) {
    for (auto& op : op_list) {
        for (auto i = 0; i < op.quantity; ++i) {
            auto elem = stack_list[op.from].front();
            stack_list[op.from].pop_front();
            stack_list[op.to].push_front(elem);
        }
    }

    std::cout << "p1: ";

    for (auto& stack : stack_list) {
        std::cout << stack.front();
    }

    std::cout << std::endl;
}

static void part2(stack_list stack_list, op_list op_list) {
    for (auto& op : op_list) {
        auto& from = stack_list[op.from];
        auto& to = stack_list[op.to];

        to.insert(to.begin(), from.begin(), from.begin()+op.quantity);
        from.erase(from.begin(), from.begin()+op.quantity);
    }

    std::cout << "p2: ";

    for (auto& stack : stack_list) {
        std::cout << stack.front();
    }

    std::cout << std::endl;
}

int main() {
    auto [layout, ops] = read_input();
    part1(layout, ops);
    part2(layout, ops);
    return 0;
}
