# Day 25: [Snowverload](https://adventofcode.com/2023/day/25)

## Part 1

**UPDATE:** [Feedback from Reddit AoC](https://www.reddit.com/r/adventofcode/comments/18qbsxs/2023_day_25_solutions/keuz4si/?context=3) was that Neato engine does a much better job at visualizing the graph.

    dot -Tpdf -Kneato input.graph > input.pdf

---

I solved this one without any code, but just using Graphviz.

First I changed the input data to make it compatible with Graphviz format.

Changed each line:

    jqt: rhn xhk nvd

into:

    jqt -- { rhn xhk nvd }

and wrapped the whole thing with `graph G {}`.

Already visualizing this one shows that there are indeed two clusters with 3 connections between them, but it's impossible to see what are the nodes.

I ran the Graphviz `cluster` tool on it, telling it to find 2 clusters:

    cluster -C2 input.graph > input_cluster.graph   

Visualizing it with the SFDP engine this time allows to read the name of the 6 nodes:

    dot -Tpdf -Ksfdp input_cluster.graph > input.pdf

I manually removed them from the file:

    kkp -- vtv
    jll -- lnf
    cmj -- qhd

The ran again `cluster` on it:

    cluster -C2 input_cluster_split.graph > input_cluster_split_cluster.graph

And just counted how many nodes are marked in cluster 1 and 2:

    grep -c "cluster=1" input_cluster_split_cluster.graph
    grep -c "cluster=2" input_cluster_split_cluster.graph

Multiply the total *et voilà*!

## Part 2

Part 2 actually requires to have solved all other days fully, which I haven't done yet.