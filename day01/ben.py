def extract_calories(input_file: str = "input.txt") -> list[int]:
    """List comprehension to extract per-elf calorie sum for the input file"""
    with open(input_file) as f:
        calories = [
            sum(
                [int(cal) for cal in cals.split("\n")]
            )  # Sum the line-by-line calorie value for an individual elf
            for cals in f.read().split(
                "\n\n"
            )  # Split the input on double-line breaks to create a list of list of values for each elf
        ]

    return calories


def solve_part_1(calories: list[int]) -> int:
    """Simple helper function to find the highest calorie count"""
    return max(calories)


def solve_part_2(calories: list[int]) -> int:
    """Extract and sum the top 3 highest values in a list"""
    sorted_calories = sorted(calories)  # Sort the list in an ascending order
    top_3 = sorted_calories[-3:]  # Retrieve the final 3 elements
    top_3_sum = sum(top_3)  # Sum the final 3 elements

    return top_3_sum


if __name__ == "__main__":
    calories = extract_calories("input.txt")

    max_cals = solve_part_1(calories)
    print(f"Part 1: The mightiest elf is carrying {max_cals} calories worth of food!")

    top_three = solve_part_2(calories)
    print(
        f"Part 2: The three most dilligent elves are carrying a total of {top_three} calories!"
    )
