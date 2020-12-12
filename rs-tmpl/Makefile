test: main.rs leetcode.rs
	rustc --edition=2018 --color=always -Adead_code -Aunused_variables -Aunused_imports -o $@ main.rs

run: test
	@echo "\033[32mbuild ok\033[0m"
	@./test

clean:
	rm test
