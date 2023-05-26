build: pdf

pdf: main.tex out
	pdflatex --jobname=Logic_Surkis_Anton_HW11 --shell-escape --output-directory=out $<

out:
	mkdir -p out

clean:
	rm -rf out
