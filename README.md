# Lecture Slides for IEEE Rust Workshop
Lecture slides for IEEE Rust workshop October 19, 2023 and October 26, 2023 generated using Latex beamer package.

## Requirements
The slides are generated using Latex. To compile the source, ensure that a Latex distribution is installed on your system.

### Latex Packages

- [`listings`](https://www.ctan.org/pkg/listings "listings package on ctan")
- [`beamer`](https://www.ctan.org/pkg/beamer "beamer package on ctan") 
- [`tcolorbox`](https://www.ctan.org/pkg/tcolorbox "tcolorbox package on ctan") 
- [`graphicx`](https://www.ctan.org/pkg/graphicx "graphicx package on ctan") 
- [`xcolor`](https://www.ctan.org/pkg/xcolor "xcolor package on ctan") 
- [`babel`](https://www.ctan.org/pkg/babel "babel package on ctan") 
- [`inputenc`](https://www.ctan.org/pkg/inputenc "inputenc package on ctan") 
- [`fontenc`](https://www.ctan.org/pkg/fontenc "fontenc package on ctan") 

## Compilation
To compile all the pdfs ensure that your have a Latex distribution installed. Once installed run:

``` sh
make all
```

This will generate all the pdf files in the directory `slides`. However this can be changed in the `Makefile` using the `OUTPUTDIR` variable.

### Creating Archives

The `Makefile` also provide a convinient way to archive the output directory.

zip: `make zip`

tar.gz: `make tar`

### Cleaning

To clean the output run: `make clean`

## License
<a rel="license" href="http://creativecommons.org/licenses/by-nc-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-sa/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc-sa/4.0/">Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License</a>.
