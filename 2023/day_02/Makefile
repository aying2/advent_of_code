CXX = clang++
CXXFLAGS = -std=c++17 -Weverything -Wno-c++98-compat -Wno-missing-prototypes

main: main.cpp
	$(CXX) -o $@ $^ $(CXXFLAGS)

.PHONY: clean
clean:
	rm *.o
