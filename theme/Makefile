# Makefile for book.js patch management
# Provides convenient commands for maintaining the patch system

.PHONY: help apply-patches verify-patches extract-patches update-mdbook clean test-syntax

# Default target
help:
	@echo "Book.js Patch Management Commands:"
	@echo ""
	@echo "  apply-patches     - Apply all patches to create book.js"
	@echo "  verify-patches    - Verify patches apply cleanly"
	@echo "  extract-patches   - Extract patches from current book.js"
	@echo "  update-mdbook     - Update to new mdbook version (requires VERSION=vX.Y.Z)"
	@echo "  test-syntax      - Test JavaScript syntax"
	@echo "  clean            - Clean temporary files"
	@echo "  help             - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make apply-patches"
	@echo "  make update-mdbook VERSION=v0.4.41"
	@echo "  make verify-patches"

apply-patches:
	@echo "Applying patches..."
	@chmod +x scripts/*.sh
	@./scripts/apply-patches.sh

verify-patches:
	@echo "Verifying patches..."
	@chmod +x scripts/*.sh
	@./scripts/verify-patches.sh

extract-patches:
	@echo "Extracting patches..."
	@chmod +x scripts/*.sh
	@./scripts/extract-patches.sh

update-mdbook:
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: VERSION is required. Usage: make update-mdbook VERSION=v0.4.41"; \
		exit 1; \
	fi
	@echo "Updating mdbook to $(VERSION)..."
	@chmod +x scripts/*.sh
	@./scripts/update-mdbook-version.sh "$(VERSION)"

test-syntax:
	@echo "Testing JavaScript syntax..."
	@if command -v node >/dev/null 2>&1; then \
		node -c book.js && echo "✅ book.js syntax OK"; \
		node -c comprehensive-rust-enhancements.js && echo "✅ enhancements.js syntax OK"; \
	else \
		echo "Warning: node not found, skipping syntax check"; \
	fi

clean:
	@echo "Cleaning temporary files..."
	@rm -f book.js.temp book.js.backup
	@rm -f patches/current-customizations.patch
	@echo "Cleanup completed"

# Development workflow targets
dev-setup: apply-patches test-syntax
	@echo "Development setup completed"

dev-verify: verify-patches test-syntax
	@echo "Development verification completed"

# CI targets
ci-verify: verify-patches test-syntax
	@echo "CI verification completed"