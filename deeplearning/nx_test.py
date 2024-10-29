import networkx as nx
import matplotlib.pyplot as plt

nx.__version__
G = nx.star_graph(8)
plt.figure(figsize=(12,10))
nx.draw(G,with_labels=True)
plt.show()
nx.pagerank(G,alpha=0.88)

print(G.nodes)