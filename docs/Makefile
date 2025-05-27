MD_FILES=$(wildcard *.md)
OUT_DIR=out
OUT_PDF_FILES=$(patsubst %.md,$(OUT_DIR)/%.pdf,$(MD_FILES))

PANDOC=pandoc

$(OUT_DIR)/%.pdf: %.md
	mkdir -p "$(dir $@)"
	$(PANDOC) --from markdown+tex_math_dollars -V geometry:margin=1in --toc --to pdf $< -o "$@"

all: $(OUT_PDF_FILES)

clean:
	rm -rf $(OUT_DIR)

.PHONY: all clean