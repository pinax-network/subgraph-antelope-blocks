.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node http://localhost:8020 wax_blocks
	graph deploy --node=http://localhost:8020 wax_blocks

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id ???