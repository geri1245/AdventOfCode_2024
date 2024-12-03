with open("./input.txt", "r") as input_file:
    lines = input_file.readlines()

number_list1 = []
number_list2 = []
for line in lines:
    numbers = line.split()
    number_list1.append(int(numbers[0].strip()))
    number_list2.append(int(numbers[1].strip()))

number_list1.sort()
number_list2.sort()

difference_sum = 0
for i in range(0, len(number_list1)):
    difference_sum += abs(number_list1[i] - number_list2[i])

print(difference_sum)