from random import randint
import time
start = time.time()
sum = 0

rand_num = randint(1, 100)
for i in range(1_000_000):
    sum+= rand_num + rand_num

end = time.time()

print(end-start)