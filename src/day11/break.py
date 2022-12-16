num = int(input())
divs = []
for div in range(2, 24):
    while (num % div) == 0:
        num //= div
        divs.append(div)
    if div == 23:
        divs.append(num)
print(*divs)
