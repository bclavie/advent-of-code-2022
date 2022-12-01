def extract_calories(input_file: str = "input.txt") -> list[int]:
    with open(input_file) as f:
        calories = [
            sum([int(cal) for cal in cals.split("\n")])
            for cals in f.read().split("\n\n")
        ]

    return calories


def solve_part_1(calories: list[int]) -> int:
    return max(calories)


def solve_part_2(calories: list[int]) -> int:
    sorted_calories = sorted(calories)
    top_3 = sorted_calories[-3:]
    top_3_sum = sum(top_3)

    return top_3_sum


if __name__ == "__main__":
    calories = extract_calories("input.txt")

    max_cals = solve_part_1(calories)
    print(f"Part 1: The mightiest elf is carrying {max_cals} calories worth of food!")

    top_three = solve_part_2(calories)
    print(
        f"Part 2: The three most dilligent elves are carrying a total of {top_three} calories!"
    )
