
Original source: https://www.coursera.org/learn/algorithms-npcomplete/exam/WuAkl/programming-assignment-4/attempt


DISCUSSION: This assignment is deliberately open-ended, and you can implement whichever 2SAT algorithm you want.  For example, 2SAT reduces to computing the strongly connected components of a suitable graph (with two vertices per variable and two directed edges per clause, you should think through the details).  This might be an especially attractive option for those of you who coded up an SCC algorithm in Part 2 of this specialization.  Alternatively, you can use Papadimitriou's randomized local search algorithm.  (The algorithm from lecture is probably too slow as stated, so you might want to make one or more simple modifications to it --- even if this means breaking the analysis given in lecture --- to ensure that it runs in a reasonable amount of time.)  A third approach is via backtracking.  In lecture we mentioned this approach only in passing; see Chapter 9 of the Dasgupta-Papadimitriou-Vazirani book, for example, for more details.

Reduction of 2SAT to SCC
https://cp-algorithms.com/graph/2SAT.html