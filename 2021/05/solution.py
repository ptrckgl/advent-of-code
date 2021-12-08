from collections import Counter


MAX_NUM = 990  # Could probably do some more accurate checking...


def get_input():
    inp = [line.strip() for line in open("input.txt", "r")]
    lines = [[line.split()[0].split(','), line.split()[2].split(',')] for line in inp]
    return lines


def main1(diagonals=False):
    counts = [[0] * MAX_NUM for _ in range(MAX_NUM)]
    lines = get_input()

    for l in lines:
        x1, y1, x2, y2 = int(l[0][0]), int(l[0][1]), int(l[1][0]), int(l[1][1])
        if x1 == x2:
            for i in range(min(y1, y2), max(y1, y2) + 1):
                counts[x1][i] += 1

        elif y1 == y2:
            for i in range(min(x1, x2), max(x1, x2) + 1):
                counts[i][y1] += 1

        elif diagonals and abs(x1 - x2) == abs(y1 - y2):
            i, j = x1, y1
            while (abs(i - x2) >= 0 and (x1 <= i <= x2 or x2 <= i <= x1)
            and (y1 <= j <= y2 or y2 <= j <= y1)):
                counts[i][j] += 1
                i += 1 if (x2 - x1) > 0 else -1
                j += 1 if (y2 - y1) > 0 else -1

    overlapped = 0
    for row in counts:
        row_count = Counter(row)
        # Adding all values, and subtracting ones with count 0 & 1, so it counts >= 2 overlapped
        overlapped += (MAX_NUM - row_count[0] - row_count[1])

    return overlapped


if __name__ == '__main__':
    print(main1())
    print(main1(diagonals=True))
