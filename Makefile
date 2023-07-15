##
# Rust Slides
#
# @file Makefile
# @version 1.0

# Don't add the slash
OUTPUTDIR = slides
PDFLATEX = pdflatex -output-directory=../$(OUTPUTDIR)
CLEANARTIFACTS = find $(OUTPUTDIR) -not -iname "*.pdf" -not -name "$(OUTPUTDIR)"|xargs rm

.PHONY: all
all: setup.pdf basics.pdf embedded.pdf

.PHONY: zip
zip: all
	zip -r rust-slides.zip $(OUTPUTDIR)

.PHONY: tar
tar: all
	tar -czf rust-slides.tar.gz $(OUTPUTDIR)

setup.pdf: setup/setup.tex
	mkdir -p $(OUTPUTDIR)
	cd setup/ && $(PDFLATEX) setup.tex
	$(CLEANARTIFACTS)

basics.pdf: basics/basics.tex
	mkdir -p $(OUTPUTDIR)
	cd basics/ && $(PDFLATEX) basics.tex
	$(CLEANARTIFACTS)

embedded.pdf: embedded/embedded.tex
	mkdir -p $(OUTPUTDIR)
	cd embedded/ && $(PDFLATEX) embedded.tex
	$(CLEANARTIFACTS)

.PHONY: clean
clean:
	rm -rf $(OUTPUTDIR)
	rm -f *.zip
	rm -f *.tar.gz

# end
