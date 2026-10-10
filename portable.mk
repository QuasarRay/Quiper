CARGO ?= cargo
PYTHON ?= python3

.PHONY: portable-build portable-test portable-check portable-integration
portable-build:
	$(PYTHON) scripts/check-portable-packages.py build --cargo $(CARGO)
portable-test:
	$(PYTHON) scripts/check-portable-packages.py test --cargo $(CARGO)
portable-check:
	$(PYTHON) scripts/check-portable-packages.py check --cargo $(CARGO)
portable-integration: portable-build
	$(PYTHON) validation/run_portable.py --output validation/results/integer-vulkan.json
