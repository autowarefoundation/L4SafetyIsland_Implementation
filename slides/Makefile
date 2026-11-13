# Plan presentation deck.
#   make        build the PDF (two passes; the title slide uses TikZ overlays)
#   make clean  remove intermediates, keep the PDF
DECK := L4SafetyIsland-ImplementationPlan

$(DECK).pdf: $(DECK).tex ntuslides.cls $(wildcard figures/*)
	latexmk -pdf -halt-on-error -interaction=nonstopmode $(DECK).tex

clean:
	latexmk -c
	rm -f *.nav *.snm *.vrb

.PHONY: clean
