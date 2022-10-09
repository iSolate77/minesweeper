
#include <iostream>
#include <chrono>

#int main (void) {
# int n;
#  std::cout << "Enter number for n: ";
#  std::cin >> n;
#  auto start = std::chrono::high_resolution_clock::now();
#  std::cout << n*n << std::endl;
#  auto stop = std::chrono::high_resolution_clock::now();
#  auto duration = std::chrono::duration_cast<std::chrono::microseconds>(stop - start);
#  std::cout << duration.count() << " ms" << std::endl;
#}
from timeit import default_timer as timer

n = int(input("Enter number for n: "))
start = timer()
square = n * n
print(square)
end = timer()
total = (end - start) * 1000000
print("time took to execute: ", total)
