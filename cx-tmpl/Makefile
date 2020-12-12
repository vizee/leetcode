CXX=g++

test: main.cc leetcode.h
	$(CXX) -std=c++14 -o $@ main.cc

run: test
	@echo "\033[32mbuild ok\033[0m"
	@./test

clean:
	rm test
