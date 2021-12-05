awk '{if (($1)>p){sum+=1} p=$1} END {print sum-1}' day1.dat
awk '{if ((p1+p2+$1)>p){sum+=1} p=(p1+p2+$1);p2=p1;p1=$1} END {print sum-3}' day1.dat