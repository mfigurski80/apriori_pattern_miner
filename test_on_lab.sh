scp ./mining_hw2_linux $2 $1@lab0z.mathcs.emory.edu:~
ssh $1@lab0z.mathcs.emory.edu "chmod +x ./mining_hw2_linux && ./mining_hw2_linux $(basename $2) $3 $4"
scp $1@lab0z.mathcs.emory.edu:~/$4 ./$4
