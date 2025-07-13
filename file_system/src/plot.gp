# Set the output format and file name
set terminal pngcairo size 1280,720 enhanced font 'Verdana,10'
set output 'iozone_benchmark_graph.png'

# Graph titles and labels
set title "IOzone Filesystem Benchmark"
set xlabel "File Size (KB)"
set ylabel "Throughput (KB/s)"

# Enable grid and set legend position
set grid
set key top left

# --- MODIFIED PLOT COMMAND ---
# Skip the first 26 header lines in the data file.
# You might need to adjust this number slightly.
plot "output.xls" every ::26 using 1:4 with linespoints title "Write", \
     "" every ::26 using 1:6 with linespoints title "Read", \
     "" every ::26 using 1:8 with linespoints title "Random Read", \
     "" every ::26 using 1:9 with linespoints title "Random Write"
