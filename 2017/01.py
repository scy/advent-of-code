# http://adventofcode.com/2017/day/1

input = open('01.txt', 'r').read().strip()

sum = 0

# Add the last digit to the end.
input += input[0]

for pos in range(length - 1):
    if input[pos] == input[pos+1]:
        sum += int(input[pos])

print(sum)
