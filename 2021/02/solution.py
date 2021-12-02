def main1():
    hpos, depth = 0, 0
    with open("input.txt", "r") as fp:
        for line in fp:
            w = line.split(' ')
            if w[0] == 'forward':
                hpos += int(w[1])
            else:
                depth += int(w[1]) if w[0] == 'down' else -int(w[1])
    return hpos * depth


def main2():
    hpos, depth, aim = 0, 0, 0
    with open("input.txt", "r") as fp:
        for line in fp:
            w = line.split(' ')
            if w[0] in ['down', 'up']:
                aim += int(w[1]) if w[0] == 'down' else -int(w[1])
            else:  # forward
                hpos += int(w[1])
                depth += aim * int(w[1])
    return hpos * depth


if __name__ == '__main__':
    print(main1())
    print(main2())
