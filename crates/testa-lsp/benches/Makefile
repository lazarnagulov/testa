SIZES = 1000 10000 50000 100000 150000

fixtures:
	@mkdir -p fixtures
	@for size in $(SIZES); do \
		python3 generator.py --lines $$size --output fixtures/fixture_$$size.testa; \
	done

bench: fixtures
	cargo bench