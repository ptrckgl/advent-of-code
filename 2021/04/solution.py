BOARD_SIZE = 5

def get_input():
    boards = []
    inp = [line.strip() for line in open("input.txt", "r")]
    nums, lines = inp[0], inp[2:]
    
    while len(lines) > 0:
        boards.append([line.split() for line in lines[0:BOARD_SIZE]])
        lines = lines[BOARD_SIZE + 1:]
     
    return nums, boards


def check_bingo(board):
    count = 0
    for row in board:
        for val in row:
            count += 1 if val[0] == '-' else 0
        if count == 5:
            return True
        count = 0


def calculate_score(board, final_num):
    total = 0
    for row in board:
        for val in row:
            total += int(val) if val[0] != "-" else 0
    return int(total) * int(final_num)


def main1(final_board=False):
    nums, boards = get_input()
    final_board_score = 0
    for num in list(map(str, nums.split(','))):
        ind = 0
        while ind < len(boards):  # Have to use 'while' since modifying the list whilst iterating
            for b, row in enumerate(boards[ind]):
                for i, n in enumerate(row):
                    if row[i] == num:
                        # Mark a number by making it negative
                        boards[ind][b][i] = f"-{row[i]}"                       
       
            if check_bingo(boards[ind]) or check_bingo(list(zip(*boards[ind]))):
                if final_board:
                    final_board_score = calculate_score(boards[ind], num)
                    del boards[ind]
                    ind -= 1
                else:
                    return calculate_score(boards[ind], num)
            ind += 1

    return final_board_score


def main2():
    return main1(final_board=True)


if __name__ == '__main__':
    print(main1())
    print(main2())
