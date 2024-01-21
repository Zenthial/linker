#include <stdio.h>

const int CONSTANT = 2;

int fib(int n) {
  if (n > 1) {
    return fib(n - 2) + fib(n - 1);
  } else {
    return n;
  }
}

int main() {
  printf("%d %d", fib(5), CONSTANT);
  return 0;
}
