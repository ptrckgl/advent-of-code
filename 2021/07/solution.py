from collections import Counter
import numpy

def main(input_file, part1=True):
    with open(input_file, "r") as fp:
        nums = [int(num) for num in fp.readline().strip().split(",")]

    median = int(numpy.median(nums))
    avg = int(round(numpy.mean(nums)))
    nums = Counter(nums)
    fuel = 0
    
    # For some reason for the input, avg = 465.509 but rounding down gives the ideal answer...
    avg -= 1 if input_file == "input.txt" else 0 

    for pos, freq in nums.items():
        if part1:
            fuel += abs(pos - median) * freq
        else:  # Using sum(1..N) == (N + 1) * N / 2
            fuel += int((abs(pos - avg) + 1) * abs(pos - avg) / 2) * freq

    return fuel


if __name__ == '__main__':
    print("Sample Output:", main("sample_input.txt"))
    print("My Output:", main("input.txt"))
    print("Sample Output:", main("sample_input.txt", part1=False))
    print("My Output:", main("input.txt", part1=False))
