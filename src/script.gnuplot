unset logscale x
unset logscale y
unset logscale xy
set xlabel "nth piece of work completed"
set ylabel "Mark (%)"
set ytics add ("MEAN" MEAN)
set mytics 1
set grid
set style line 4 lw 3
set datafile separator ","
set key font ",15"
set terminal png size 1024,768
set output "OUTPUTFILEPATH"
b = 0.0
m = 0.0
l(x) = m*x + b
fit l(x) "INPUTFILEPATH" u 1:4 via b, m
plot "INPUTFILEPATH" u 1:4 w linespoints lc "COLOUR" lw 2 title "SUBJNAME",\
     l(x) lw 3 title "LOBF",\
     "INPUTFILEPATH" u 1:5 w lines lw 3 title "Mean average"