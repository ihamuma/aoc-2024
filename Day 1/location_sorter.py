file = open('Day 1/input.txt', 'r')
data = file.readlines()

data_split = [tuple(x.strip('\n').split('   ')) for x in data]
locations_1, locations_2 = map(list, zip(*data_split))

locations_1 = [int(x) for x in locations_1]
locations_2 = [int(x) for x in locations_2]

locations_1.sort()
locations_2.sort()

distances = [abs(locations_1[i] - locations_2[i]) for i in range(0, len(locations_1))]

print(sum(distances))