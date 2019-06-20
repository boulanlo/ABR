#!/bin/bash

sizes=$(ls bench_results -1 | sed -e 's/\..*$//' | uniq);
bench_folder="bench_results"
config_file="$bench_folder/gnuplot.conf"

cat <<EOF > $config_file
set terminal jpeg
set style line 1 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 0.5
EOF

for s in $sizes; do
    const=$(cat "$bench_folder/$s".constant);

    cat <<EOF >> $config_file
set output "$bench_folder/$s.jpg"
set title 'Sum of a random tree of size $s, with and without levels, 3 threads'
set xlabel 'Level used'
set ylabel 'Time (ns)'
set style fill transparent solid 0.5 noborder
plot "$bench_folder/$s.data" with linespoints linestyle 1 title "With levels(x)", $const with lines title "Without levels"
EOF
    
done;

cat $config_file | gnuplot;
