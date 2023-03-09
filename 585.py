from tqdm import tqdm
from matplotlib import pyplot as plt
import time

# for runtime estimation
import numpy as np
from scipy.optimize import curve_fit


def is_square(n):
    return int(n**0.5) == n**0.5

def is_denested(x, y, z, roots):
    right_term = (x + y**0.5 + z**0.5)**0.5
    left_term = sum(i**0.5 for i in roots)
    return right_term == left_term

"""

x + root(y) + root(z) = sum(a_i) + 2*sum(root(a_i * a_j))

x = a+b

root(y) + root(z) = 2*root(a*b)

4*a*b = y + z + 2*root(y*z)

b = (y + z + 2*root(y*z)) / (4*a)

x = a + (y + z + 2*root(y*z)) / (4*a) 

a - x =  -1 * (y + z + 2*root(y*z)) / (4*a) 

4 (a^2 - xa ) = -1 * (y + z + 2*root(y*z)) 





"""


def f(n):
    m = {}
    combos = []
    for x in tqdm(range(1, n)):
        for a in range(0, x//2 + 1):

            b = x - a
                
            l = [a, b]
            l.sort()

            if str(l) in m:
                continue

            for y in range(x):

                # no perfect squares
                if is_square(y):
                    continue

                z1 = -4 * y**0.5 * (a*b)**0.5 + 4*a*b + y
                z2 = 4 * y**0.5 * (a*b)**0.5 + 4*a*b + y

                for z in [z1, z2]:

                    # no perfect squares
                    if  z < 0 or is_square(z) or int(z) != z:
                        continue

                    z = int(z)
                    
                    if is_denested(x, y, z, [a, b]):
                        s = "x:{} y:{} z:{} a:{} b:{}"
                        combos.append(s.format(x, y, z, a, b))
                        m[str(l)] = True
                        
    for p in combos:
        print(p)

    return len(m.keys())


 # Define the function you want to fit
def func(n, a, b, c):
    return a * n**2 + b * n + c


def main():
    
    out = f(5000000)
    if out != 17:
        print('-'*30)
        print('got {} instead of 17'.format(out))
        print(' ')
        print('failed...')
        print('-'*30)

    return

    times = []
    for i in range(20):
        t = time.time()
        f(i)
        times.append(time.time()-t)

    # Define the data you have collected
    n_values = np.array([i for i in range(20)])
    times = np.array(times)

    # Fit the function to the data
    popt, pcov = curve_fit(func, n_values, times)

    # Estimate the runtime for n=5000000
    runtime = round(func(5000000, *popt)/(60*60), 2)
    print("Estimated runtime for n=5000000: {} hours".format(runtime))

    est_times = []
    est_ns = []
    for i in range(20, 50):
        est_ns.append(i)
        est_times.append(func(i, *popt))

    plt.plot(times)
    plt.plot(est_ns, est_times, linestyle='--')
    plt.title("Algo Time Complexity")
    plt.xlabel("n")
    plt.ylabel('t(n)')
    plt.show()

    

if __name__ == '__main__':
    main()