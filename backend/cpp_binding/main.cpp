#include <iostream>

using i64 = long long;

extern "C" {
    i64 sum(i64, i64, i64);
}

int main() {
    std::cout << sum(1, 2, 3);
}
