import math 
input = input('')

def solve_quadractic(a, b, c):
    delta = b**2 - 4*a*c

    if delta < 0 or a == 0:
        print('Impossivel calcular')
    else:
        r1 = (-b + math.sqrt(delta)) / (2*a)
        r2 = (-b - math.sqrt(delta)) / (2*a)

        print(f'R1 = {r1:.5f}\nR2 = {r2:.5f}')


input = input.split()
a = float(input[0])
b = float(input[1])
c = float(input[2])

solve_quadractic(a, b, c)