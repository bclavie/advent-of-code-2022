def lookup_lambda(scores_lookup, input_file: str = "input.txt") -> int:
    return next(
        int(sum(x) / 2) for x in zip(*map(scores_lookup, open(input_file).readlines()))
    )


def solve_part_1(input_file: str = "input.txt") -> int:
    return lookup_lambda(
        lambda x: ("  BXCYAZAXBYCZCXAYBZ".index(x[0] + x[2]),),
        input_file,
    )


def solve_part_2(input_file: str = "input.txt") -> int:
    return lookup_lambda(
        lambda x: ("  BXCXAXAYBYCYCZAZBZ".index(x[0] + x[2]),),
        input_file,
    )


print(solve_part_1())
print(solve_part_2())
