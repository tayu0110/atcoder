#include <iostream>

class C {
	int val;
public:
	C(int val) : val{val} {}
	C operator+(const C &rhs) {
		return C(val + rhs.val);
	}
	C &operator=(const C rhs) = delete;
	C &operator=(const C &rhs) = delete;
	friend std::ostream &operator<<(std::ostream &os, const C c) {
		os << c.val;
		return os;
	}
};

int main() {
	C c = C(10);
//	c = C(20);		// NG (Compile Error)
	C d = c;				// OK
	std::cout << "c: " << c << " d: " << d << std::endl;
//	d = C(30);		// NG (Compile Error)
	C e = c + d;		// OK
	std::cout << "e: " << e << std::endl;
//	e = d;				// NG (Compile Error)
	return 0;
}
