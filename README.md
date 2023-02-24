# compress_output_of_bedtools_coverage

## GOAL
when using bedtools to count per base coverage of feature the output is extremly large.
You can pipe the output of bedtools coverage -d to this tools and will compress it by a factor ~20 (see README for detail).

For now only cover the following use case:
When conting coverage from a bam to genome feature using bedtools bamtobed and coverage

example:


## installation
clone this directory, then cd inside and cargo -build --release

the execuable will then be ./target/release/pipe_bedtools

## Usage
When conting coverage from a bamfile to genome feature (in a bedfile) using bedtools bamtobed and coverage


`

    bedtools bamtobed -i ${bamfile} | \
    
    bedtools coverage -sorted -s -split  -b - -a ${bedfile} -d | \ 
    
    ./target/release/pipe_bedtools > genes.bed.forward.coverage.bg
`
# sorting bamfile, bedfile and usage of the sorted option is recommanded for keeping memory consomption low




## Specification

### Output

the outptut is the 6 columns of the bed file + 3 columns. start, end depth. start and end define and interval which has a given depth.
value are grouped by the attribute "name" i.e. the 4th field of the bedfile.
ex:
```
 
chr2L   59190   59268   FBgn0051973_FBtr0309228_exon_1  0       -       1       77      30 # from first base the the 77th base the depth is 30

chr2L   66482   66612   FBgn0067779_FBtr0306539_exon_1  0       +       1       11      80 # from first base the the 11 base the depth is 80
chr2L   66482   66612   FBgn0067779_FBtr0306539_exon_1  0       +       11      30      40 # from 11th base the the 30th base the depth is 40
chr2L   66482   66612   FBgn0067779_FBtr0306539_exon_1  0       +       30      45      98
chr2L   66482   66612   FBgn0067779_FBtr0306539_exon_1  0       +       46      46      25
chr2L   66482   66612   FBgn0067779_FBtr0306539_exon_1  0       +       34      70      54
chr2L   66482   66612   FBgn0067779_FBtr0306539_exon_1  0       +       70      150     54
chr2L   66482   66612   FBgn0067779_FBtr0306539_exon_1  0       +       150     190     10

```





