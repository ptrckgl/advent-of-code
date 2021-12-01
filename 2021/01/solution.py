# Part 1 Solution
def main1():
    prev = -1
    increased = 0
    with open("input.txt", "r") as fp:
        for line in fp:
            increased += 1 if int(line) > prev else 0
            prev = int(line)

    return increased - 1  # As prev is initially -1


# Part 2 Solution
def main2():
    vals = [-1, -1, -1]
    prevsum = 0
    increased = 0
    with open("input.txt", "r") as fp:
        for line in fp:
            prevsum = sum(vals)
            vals[0], vals[1], vals[2] = vals[1], vals[2], int(line)
            increased += 1 if prevsum < sum(vals) and -1 not in vals else 0

    return increased - 1


if __name__ == '__main__':
    print(main1())
    print(main2())
