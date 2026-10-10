CARGO ?= cargo
PYTHON ?= python3

.PHONY: portable-build portable-release portable-test portable-check portable-integration
portable-build:
	$(PYTHON) scripts/check-portable-packages.py build --cargo $(CARGO)
portable-release:
	$(PYTHON) scripts/check-portable-packages.py release --cargo $(CARGO)
portable-test:
	$(PYTHON) scripts/check-portable-packages.py test --cargo $(CARGO)
portable-check:
	$(PYTHON) scripts/check-portable-packages.py check --cargo $(CARGO)
portable-integration: portable-release
	$(PYTHON) validation/run_portable.py --release --output validation/results/integer-vulkan.json
