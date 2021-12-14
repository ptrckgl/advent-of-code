def get_input(filename):
    return [line.strip() for line in open(filename, "r")]


def main1(filename="input.txt"):
    lines = get_input(filename)
    total = 0
    for line in lines:
        sepd = line.split(' ')[11:]
        for num in sepd:
            # 2, 3, 4, 7 are the lengths of 1, 7, 4, 8 respectively
            total += 1 if len(num) in [2, 3, 4, 7] else 0

    return total


def decode_letters(line):
    """Decodes the letters in the seven-segment display, learning which letter maps to which"""
    decoded_map = {2: [], 3: [], 4: [], 5: [], 6: [], 7: []}
    letter_map = {}
    sepd = line.split(' ')
    for num in sepd[:10]:
        decoded_map[len(num)].append(num)

    # Notice that '7' contains the same letters as '1' except for 1
    letter_map['a'] = list({letter for letter in decoded_map[3][0]} - {letter for letter in decoded_map[2][0]})[0]

    # '3' Contains same letters as '4', plus 'a' and 'g'. We know 'a' already!
    # We can uniquely identify '3' as it has the same characters as '1'
    temp = ''
    for num in decoded_map[5]:
        if len(list(set(num) - set(decoded_map[4][0]) - set(letter_map['a']))) == 1:
            # The letters represent the number '3' or '5' (both work in this case)
            letter_map['g'] = list(set(num) - set(decoded_map[4][0]) - set(letter_map['a']))[0]
            break

    # 'e' is number '8' - number '4' - 'a' - 'g'
    letter_map['e'] = list(set(decoded_map[7][0]) - set(decoded_map[4][0]) - set(letter_map['a']) - set(letter_map['g']))[0]

    for num in decoded_map[5]:
        # '2' is the only number which has both letters 'e' and 'g' in them for numbers of length 5
        if len(list(set(num) - set(letter_map['e']) - set(letter_map['g']))) == 3:
            # The letters represent the number '2'
            letter_map['d'] = list(set(num) - set(letter_map['g']) - set(letter_map['e']) - set(letter_map['a']) - set(decoded_map[2][0]))[0]
            letter_map['c'] = list(set(num) - set(letter_map['a']) - set(letter_map['d']) - set(letter_map['e']) - set(letter_map['g']))[0]
    
    letter_map['f'] = list(set(decoded_map[2][0]) - set(letter_map['c']))[0]
    letter_map['b'] = list(set(decoded_map[4][0]) - set(letter_map['c']) - set(letter_map['d']) - set(letter_map['f']))[0]

    return letter_map


def get_mapped_num(letter_map, num):
    """Maps the 'num' value into a new value based on the letter_map dict"""
    # What we are doing essentially is finding which value maps to each digit in num,
    # And then adding 'key' value to the new_num string
    new_num = ''
    for digit in num:
        for key, val in letter_map.items():
            if val == digit:
                new_num += key
                break
    return new_num


def main2(filename="input.txt"):
    # Note: We know that there is one number of length 2, 3, 4, 7 in each line
    # Use this information to deduce the rest of the numbers
    lines = get_input(filename)
    total = 0
    number_map = {
        0: 'abcefg',
        1: 'cf',
        2: 'acdeg',
        3: 'acdfg',
        4: 'bcdf',
        5: 'abdfg',
        6: 'abdefg',
        7: 'acf',
        8: 'abcdefg',
        9: 'abcdfg'
    }

    for line in lines:
        letter_map = decode_letters(line)
        digits = ''
        sepd = line.split(' ')
        for num in sepd[11:]:
            mapped_num = get_mapped_num(letter_map, num)
            for n in range(0, 10):
                if set(mapped_num) == set(number_map[n]):
                    digits += str(n)
                    break
        total += int(digits)
    
    return total


if __name__ == '__main__':
    print(main1())
    print(main2(filename="sample_input.txt"))
    print(main2())
