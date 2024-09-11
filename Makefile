RUN = poetry -C backend run
VERSION = $(shell poetry -C backend version -s)
ROOTDIR = $(shell pwd)
SCHEMADIR = $(ROOTDIR)/backend/src/monarch_py/datamodels

### Help ###
.PHONY: help
help:
	@echo "╭───────────────────────────────────────────────────────────╮"
	@echo "│ Makefile for Monarch API                                  │"
	@echo "│ ────────────────────────                                  │"
	@echo "│ Usage:                                                    │"
	@echo "│     make <target>                                         │"
	@echo "│                                                           │"
	@echo "│ Targets:                                                  │"
	@echo "│     help                Print this help message           │"
	@echo "│     all                 Install everything                │"
	@echo "│     fresh               Clean and install everything      │"
	@echo "│     clean               Clean up build artifacts          │"
	@echo "│     clobber             Clean up generated files          │"
	@echo "│                                                           │"
	@echo "│     docs                Generate documentation            │"
	@echo "│     model               Generate model files              │"
	@echo "|     fixtures            Generate data fixtures            │"
	@echo "│                                                           │"
	@echo "│     install             Install backend and frontend      │"
	@echo "│     install-backend     Install backend                   │"
	@echo "│     install-frontend    Install frontend                  │"
	@echo "│                                                           │"
	@echo "│     test                Run all tests                     │"
	@echo "│     test-backend        Run backend tests                 │"
	@echo "│     test-frontend       Run frontend tests                │"
	@echo "│                                                           │"
	@echo "│     dev-frontend        Run frontend in development mode  │"
	@echo "│     dev-api             Run api in development mode       │"
	@echo "│                                                           │"
	@echo "│     docker-build        Build docker image                │"
	@echo "│     docker-push         Push docker image                 │"
	@echo "│                                                           │"
	@echo "│     lint                Lint all code                     │"
	@echo "│     lint-backend        Lint backend code                 │"
	@echo "│     lint-frontend       Lint frontend code                │"
	@echo "│                                                           │"
	@echo "│     format              Format all code                   │"
	@echo "│     format-backend      Format backend code               │"
	@echo "│     format-frontend     Format frontend code              │"
	@echo "╰───────────────────────────────────────────────────────────╯"

### Installation and Setup ###

.PHONY: fresh
fresh: clean clobber all


.PHONY: all
all: install model docs


.PHONY: install
install: install-backend install-frontend


.PHONY: install-backend
install-backend:
	cd backend && \
		cargo build --release
		# prob move the binary somewhere and then clean idk


.PHONY: install-frontend
install-frontend:
	cd frontend && \
		npm install


.PHONY: model
model:
	$(RUN) gen-typescript model.yaml > frontend/src/api/model.ts


### Documentation ###

docs/Data-Model:
	mkdir -p $@

.PHONY: docs
docs: install-backend docs/Data-Model
	$(RUN) gen-doc -d $(ROOTDIR)/docs/Data-Model/ model.yaml
	$(RUN) typer backend/src/monarch_py/cli.py utils docs --name monarch --output docs/Usage/CLI.md
	$(RUN) mkdocs build


### Testing ###

.PHONY: test
test: test-backend test-frontend


.PHONY: test-backend
test-backend: 
	# run backend tests


.PHONY: test-frontend
test-frontend: 
	# run frontend tests


### Development ###

.PHONY: dev-frontend
dev-frontend:
	cd frontend && \
		npm run dev


.PHONY: dev-backend
dev-backend: 
	cd backend && \
		RUST_BACKTRACE=1 cargo run --quiet


### Linting, Formatting, and Cleaning ###

.PHONY: clean
clean:
	mv backend/target/release/whenworks-backend $(ROOTDIR)/ &&
		rm -rf backend/target/


.PHONY: clobber
clobber:
	rm -f \
		frontend/src/api/model.ts


.PHONY: lint
lint: lint-frontend lint-backend


.PHONY: lint-frontend
lint-frontend: 
	# run frontend linters


.PHONY: lint-backend
lint-backend: 
	cd backend && \
		cargo clippy


.PHONY: format
format: format-frontend format-backend


.PHONY: format-backend
format-backend: 
	echo "don't"


.PHONY: format-frontend
format-frontend:
	# run frontend formatters
