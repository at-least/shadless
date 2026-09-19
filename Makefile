# shadless — pipeline orchestration
#
# Every build step and gate is a node of the graph the Rust engine at
# pipeline/ defines; the engine executes the topologically sorted closure of
# what you ask for. This file only names the common entry points — it holds
# no ordering of its own, so it cannot drift from what CI runs.
#
#   make              full pipeline + every gate
#   make verify       every gate, assuming artifacts are fresh
#   make fast         browser-free gates                    (< 1s, pre-commit)
#   make meta         prove every gate can fail (mutation testing)
#   make only ID=x    one node + exactly what it needs      (make only ID=path-parity)
#   make list         the graph
#   make all          the same graph, every node, no freshness skip
#
# build/fast/only go through the engine (pipeline/): a node whose
# declared inputs and dependencies are unchanged since its last green run is
# skipped, and independent nodes run in parallel. PIPELINE_PARALLEL caps
# concurrency — playwright nodes each own a chromium. The freshness record is
# one file per node under pipeline/stamps/, and it is NOT tracked: a fresh
# clone holds the committed outputs but no record of which inputs produced
# them, so it starts cold.
#
# PIPELINE_PARALLEL=1 additionally enables the undeclared-WRITE check, which
# needs a quiet tree to attribute a change to the node that made it.
#   make upstream TO=shadcn@X.Y.Z   re-pin drill: pin, dissolve, rebuild, classify
#   make overlay-tasks              task packets for stale/orphaned manual work
#   make serve / clean

.DEFAULT_GOAL := build
NPM    := npm
PYTHON := python3
PORT   ?= 8765
PIPELINE_PARALLEL ?= 4
export PIPELINE_PARALLEL
PIPELINE := build/pipeline

.PHONY: build verify fast full all meta only list pipeline \
        pin ledger ledger-render overlay overlay-record overlay-tasks \
        upstream upstream-snapshot reproducible \
        audit-boundary ir-diff serve clean help

# ----- the pipeline -------------------------------------------------------
CRATE_SRC := $(shell find pipeline/src -name '*.rs')
$(PIPELINE): $(CRATE_SRC) pipeline/Cargo.toml pipeline/Cargo.lock pipeline/build.rs pipeline/rust-toolchain.toml
	@mkdir -p build
	cd pipeline && cargo build --release -q
	cp pipeline/target/release/pipeline $(PIPELINE)

pipeline: $(PIPELINE)

build: $(PIPELINE)
	./$(PIPELINE) run full

verify: $(PIPELINE)
	./$(PIPELINE) run all --gates-only

fast: $(PIPELINE)
	./$(PIPELINE) run fast

full: build

all: $(PIPELINE)
	./$(PIPELINE) run all --force

# Mutation testing: prove every gate can fail. Needs a built tree, runs the
# real gates, so it is opt-in rather than part of the default tier.
#   make meta ONLY=<id>   one gate's mutations
meta: $(PIPELINE)
	./$(PIPELINE) __meta $(ONLY)

only: $(PIPELINE)
	@test -n "$(ID)" || { echo "usage: make only ID=<node-id>   (make list)"; exit 2; }
	./$(PIPELINE) run $(ID)

list: $(PIPELINE)
	./$(PIPELINE) list all

# ----- ledgers -------------------------------------------------------------
# gates/ledger.json  — accepted differences, with class + budget
# overlays/manifest.json — hand-written units anchored to upstream hashes
pin:
	$(NPM) run pin

ledger: $(PIPELINE)
	./$(PIPELINE) __gate ledger

ledger-render: $(PIPELINE)
	./$(PIPELINE) ledger --render

overlay: $(PIPELINE)
	./$(PIPELINE) overlay

overlay-record: $(PIPELINE)
	./$(PIPELINE) overlay --record

overlay-tasks: $(PIPELINE)
	./$(PIPELINE) overlay --tasks

# ----- upstream ------------------------------------------------------------
# One command from "a new shadcn release exists" to "green, or a classified
# report with task packets". See UPGRADING.md.
upstream: $(PIPELINE)
	@test -n "$(TO)" || { echo "usage: make upstream TO=shadcn@X.Y.Z"; exit 2; }
	./$(PIPELINE) upstream --to=$(TO)

# Refresh the committed ui.shadcn.com snapshot (network crawl; golden hop 1).
upstream-snapshot: $(PIPELINE)
	./$(PIPELINE) upstream-snapshot

# Committed generated trees must equal a clean rebuild. CI's only authority
# on hand-edits to dist/ — the pre-commit hook no longer guesses.
reproducible: $(PIPELINE)
	./$(PIPELINE) __gate reproducible

# ----- housekeeping --------------------------------------------------------
audit-boundary: $(PIPELINE)
	./$(PIPELINE) audit-boundary

# Slot-level semantic diff between two IR sets (the re-pin review surface).
ir-diff: $(PIPELINE)
	@test -n "$(REF)" || { echo "usage: make ir-diff REF=<git-ref>"; exit 2; }
	./$(PIPELINE) ir-diff $(REF)

serve:
	@echo "serving the built docs on http://localhost:$(PORT) (Ctrl-C to stop)"
	zola --root docs/site serve --port $(PORT) -u http://127.0.0.1/

clean:
	rm -rf dist build node_modules/.cache/shadless
	rm -rf docs/catalog.json
	rm -rf docs/site/public docs/site/content docs/site/static
	@echo "cleaned: dist/ + build/ + the docs site's generated trees"

help:
	@sed -n '2,15p' Makefile | sed 's/^# \{0,1\}//'
