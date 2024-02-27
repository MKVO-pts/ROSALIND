f = open("Datasets/rosalind_revc.txt", "r").read()
map = {65: 84, 67: 71,84: 65, 71: 67}

print(f[::-1].translate(map))
