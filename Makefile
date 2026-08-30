# shadless — pipeline orchestration
#
# Every build step and gate is a node in gates/registry.mjs; gates/run.mjs
# executes the topologically sorted closure of what you ask for. This file
# only names the common entry points — it holds no ordering of its own, so
# it cannot drift from what CI runs.
#
#   make              full pipeline + every gate            (= CI)
#   make verify       every gate, assuming artifacts are fresh
#   make fast         browser-free gates                    (< 1s, pre-commit)
#   make medium       + convert/emit                        (~10s, pre-push)
#   make meta         prove every gate can fail (mutation testing)
#   make only ID=x    one node + exactly what it needs      (make only ID=path-parity)
#   make list         the graph
#   make all          the same graph, every node, no freshness skip (gates/run.mjs)
#
# build/fast/medium/only go through the Go runner (pipeline/): a node whose
# declared inputs and dependencies are unchanged since its last green run is
# skipped, and independent nodes run in parallel. PIPELINE_PARALLEL caps
# concurrency — playwright nodes each own a chromium. The freshness record is
# pipeline/stamps.json, which is TRACKED, so a fresh clone is already warm for
# every node whose outputs are committed.
#   make upstream TO=shadcn@X.Y.Z   re-pin drill: pin, dissolve, rebuild, classify
#   make overlay-tasks              task packets for stale/orphaned manual work
#   make serve / clean / hooks

.DEFAULT_GOAL := build
NODE   := node
NPM    := npm
PYTHON := python3
PORT   ?= 8765
PIPELINE_PARALLEL ?= 4
export PIPELINE_PARALLEL
PIPELINE := build/pipeline

.PHONY: build verify fast medium full all meta only list pipeline \
        pin ledger ledger-render overlay overlay-record overlay-tasks \
        upstream upstream-snapshot reproducible \
        hooks hooks-uninstall audit-boundary serve clean help

# ----- the pipeline -------------------------------------------------------
$(PIPELINE): $(wildcard pipeline/*.go) pipeline/go.mod
	@mkdir -p build
	cd pipeline && go build -o ../$(PIPELINE) .

pipeline: $(PIPELINE)

build: $(PIPELINE)
	./$(PIPELINE) run full

verify:
	$(NODE) gates/run.mjs --tier=full --gates-only

fast: $(PIPELINE)
	./$(PIPELINE) run fast

medium: $(PIPELINE)
	./$(PIPELINE) run medium

full: build

all:
	$(NODE) gates/run.mjs --tier=full

meta:
	$(NODE) gates/meta.mjs

only: $(PIPELINE)
	@test -n "$(ID)" || { echo "usage: make only ID=<node-id>   (make list)"; exit 2; }
	./$(PIPELINE) run $(ID)

list:
	$(NODE) gates/run.mjs --tier=full --list

# ----- ledgers -------------------------------------------------------------
# gates/ledger.json  — accepted differences, with class + budget
# overlays/manifest.json — hand-written units anchored to upstream hashes
pin:
	$(NPM) run pin

ledger:
	$(NODE) gates/ledger.mjs --verify

ledger-render:
	$(NODE) gates/ledger.mjs --render

overlay:
	$(NODE) gates/overlay.mjs --audit

overlay-record:
	$(NODE) gates/overlay.mjs --record

overlay-tasks:
	$(NODE) gates/overlay.mjs --tasks

# ----- upstream ------------------------------------------------------------
# One command from "a new shadcn release exists" to "green, or a classified
# report with task packets". See UPGRADING.md.
upstream:
	@test -n "$(TO)" || { echo "usage: make upstream TO=shadcn@X.Y.Z"; exit 2; }
	$(NODE) gates/upstream.mjs --to=$(TO)

# Refresh the committed ui.shadcn.com snapshot (network crawl; golden hop 1).
upstream-snapshot:
	$(NODE) tools/upstream-snapshot.mjs

# Committed generated trees must equal a clean rebuild. CI's only authority
# on hand-edits to dist/ and docs/site/ — the pre-commit hook no longer guesses.
reproducible:
	go test -C pipeline -count=1 -v -run '^TestReproducible$$' .

# ----- housekeeping --------------------------------------------------------
hooks:
	$(NODE) tools/git-hooks/install.mjs

hooks-uninstall:
	$(NODE) tools/git-hooks/install.mjs --uninstall

audit-boundary:
	$(NODE) tools/audit-boundary.mjs

serve:
	@echo "serving docs/site/ on http://localhost:$(PORT) (Ctrl-C to stop)"
	cd docs/site && $(PYTHON) -m http.server $(PORT)

clean:
	rm -rf dist build node_modules/.cache/shadless
	rm -rf docs/catalog.json
	rm -f  docs/site/site.css docs/site/site.js docs/site/highlight.js docs/site/out.css docs/site/fonts.css
	rm -rf docs/site/assets docs/site/components docs/site/js
	rm -f  docs/site/shadless.js
	rm -f  docs/site/*.html
	@echo "cleaned: dist/ + build/ + docs/site/ generated artifacts"

help:
	@sed -n '2,16p' Makefile | sed 's/^# \{0,1\}//'
