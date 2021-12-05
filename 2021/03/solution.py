BIN_LEN = 12  # Assume binary number length of 12

def determine_vals(input_list):
    vals = [0 for i in range(0,BIN_LEN)]
    for line in input_list:
        for index, ch in enumerate(line.strip()):
            vals[index] += int(ch)
    return (vals, len(input_list))


def main1():
    vals, lines = determine_vals([line for line in open("input.txt", "r")])

    gamma, epsilon = 0, 0
    for index, val in enumerate(vals):
        gamma += (val > lines / 2) * (2 ** (len(vals) - index - 1))
        epsilon += (val < lines / 2) * (2 ** (len(vals) - index - 1))
    
    return gamma * epsilon


def main2():
    bit_index, ogr, csr = 0, 0, 0
    ogr_vals, ogr_lines = determine_vals([line for line in open("input.txt", "r")])
    csr_vals, csr_lines = ogr_vals.copy(), ogr_lines
    nums = [line.strip() for line in open("input.txt", "r")]
    ogr_nums, csr_nums = nums.copy(), nums.copy()

    while bit_index < 12 and not (ogr != 0 and csr != 0):
        i = 0
        while i < len(ogr_nums) and len(ogr_nums) > 1:
            if not ((ogr_nums[i][bit_index] == '1' and ogr_vals[bit_index] >= ogr_lines / 2) or
                    (ogr_nums[i][bit_index] == '0' and ogr_vals[bit_index] < ogr_lines / 2)):
                del ogr_nums[i]
                i -= 1
            i += 1

        i = 0
        while i < len(csr_nums) and len(csr_nums) > 1:
            if not ((csr_nums[i][bit_index] == '0' and csr_vals[bit_index] >= csr_lines / 2) or
                    (csr_nums[i][bit_index] == '1' and csr_vals[bit_index] < csr_lines / 2)):
                del csr_nums[i]
                i -= 1
            i += 1
        
        ogr = ogr_nums[0] if len(ogr_nums) == 1 and ogr == 0 else 0
        if len(csr_nums) == 1 and csr == 0:  # For some reason doesn't work in a single line??
            csr = csr_nums[0]
        
        bit_index += 1
        ogr_vals, ogr_lines = determine_vals(ogr_nums)
        csr_vals, csr_lines = determine_vals(csr_nums)

    return int(str(ogr), 2) * int(str(csr), 2)


if __name__ == '__main__':
    print(main1())
    print(main2())
