# Lecture Slides for IEEE Rust Workshop
Lecture slides for IEEE Rust workshop generated using Latex beamer package.

## Requirements
The slides are generated using Latex. To compile the source, ensure that a Latex distribution is installed on your system.

### Latex Packages
`beamer`

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
