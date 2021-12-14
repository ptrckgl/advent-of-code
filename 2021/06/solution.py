from collections import Counter

def main(num_days):
    with open("input.txt", "r") as fp:
        nums = [num for num in fp.readline().strip().split(',')]
    nums_ctr = Counter(nums)

    for val in range(0, 9):
        if not nums_ctr[str(val)]:
            nums_ctr[str(val)] = 0
    
    for days in range(0, num_days):
        zero_num = nums_ctr["0"]
        for num in sorted(nums_ctr):
            if num != "0":
                nums_ctr[str(int(num) - 1)] += nums_ctr[num]
                nums_ctr[num] = 0
        
        # Reproduce
        nums_ctr["0"] -= zero_num
        nums_ctr["6"] += zero_num
        nums_ctr["8"] += zero_num

    return(sum([num[1] for num in nums_ctr.items()]))


if __name__ == '__main__':
    print(main(80))
    print(main(256))
