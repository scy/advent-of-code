# http://adventofcode.com/2018/day/1

file = open('02.txt', 'r')

sum = 0

for line in file.readlines():
    values = [int(x) for x in line.strip().split("\t")]
    sum += max(values) - min(values)

print(sum)
