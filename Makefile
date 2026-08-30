# shadless — pipeline orchestration
#
# Every build step and gate is a node in pipeline/nodes.go; the Go runner
# executes the topologically sorted closure of what you ask for. This file
# only names the common entry points — it holds no ordering of its own, so
# it cannot drift from what CI runs.
#
#   make              full pipeline + every gate
#   make verify       every gate, assuming artifacts are fresh
#   make fast         browser-free gates                    (< 1s, pre-commit)
#   make meta         prove every gate can fail (mutation testing)
#   make only ID=x    one node + exactly what it needs      (make only ID=path-parity)
#   make list         the graph
#   make all          the same graph, every node, no freshness skip
#
# build/fast/only go through the Go runner (pipeline/): a node whose
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
NODE   := node
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
$(PIPELINE): $(wildcard pipeline/*.go) pipeline/go.mod
	@mkdir -p build
	cd pipeline && go build -o ../$(PIPELINE) .

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
# real gates, so it is opt-in rather than part of `go test ./...`.
#   make meta TIER=fast   only mutations whose gate is browser-free
#   make meta ONLY=<id>   one mutation
meta:
	SHADLESS_META=1 META_TIER=$(TIER) META_ONLY=$(ONLY) \
	  go test -C pipeline -count=1 -v -timeout 2h -run '^TestMeta$$' .

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

ledger:
	go test -C pipeline -count=1 -v -run '^TestLedger$$' .

ledger-render: $(PIPELINE)
	./$(PIPELINE) ledger --render

overlay:
	$(NODE) gates/overlay.mjs --audit

overlay-record:
	$(NODE) gates/overlay.mjs --record

overlay-tasks:
	$(NODE) gates/overlay.mjs --tasks

# ----- upstream ------------------------------------------------------------
# One command from "a new shadcn release exists" to "green, or a classified
# report with task packets". See UPGRADING.md.
upstream: $(PIPELINE)
	@test -n "$(TO)" || { echo "usage: make upstream TO=shadcn@X.Y.Z"; exit 2; }
	./$(PIPELINE) upstream --to=$(TO)

# Refresh the committed ui.shadcn.com snapshot (network crawl; golden hop 1).
upstream-snapshot:
	$(NODE) tools/upstream-snapshot.mjs

# Committed generated trees must equal a clean rebuild. CI's only authority
# on hand-edits to dist/ — the pre-commit hook no longer guesses.
reproducible:
	go test -C pipeline -count=1 -v -run '^TestReproducible$$' .

# ----- housekeeping --------------------------------------------------------
audit-boundary: $(PIPELINE)
	./$(PIPELINE) audit-boundary

# Slot-level semantic diff between two IR sets (the re-pin review surface).
ir-diff: $(PIPELINE)
	@test -n "$(REF)" || { echo "usage: make ir-diff REF=<git-ref>"; exit 2; }
	./$(PIPELINE) ir-diff $(REF)

serve:
	@echo "serving docs/site/ on http://localhost:$(PORT) (Ctrl-C to stop)"
	cd docs/site && $(PYTHON) -m http.server $(PORT)

clean:
	rm -rf dist build node_modules/.cache/shadless
	rm -rf docs/catalog.json
	rm -rf docs/site
	@echo "cleaned: dist/ + build/ + docs/site/ generated artifacts"

help:
	@sed -n '2,16p' Makefile | sed 's/^# \{0,1\}//'
