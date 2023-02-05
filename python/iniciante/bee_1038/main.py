input = input('')

code = int(input.split()[0])
qtd = int(input.split()[1])

price = 0
if code == 1:
    price = 4
elif code == 2:
    price = 4.5
elif code == 3:
    price = 5
elif code == 4:
    price = 2
elif code == 5:
    price = 1.5

total = price*qtd
print(f'Total: R$ {total:.2f}')