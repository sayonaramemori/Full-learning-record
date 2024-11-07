import networkx as nx
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import sys
import os
sys.path.append(os.getcwd()+R'\GNN')
from utils import sort_utils

# G = nx.complete_graph(7,nx.DiGraph())
# G = nx.complete_graph(7)
# nx.draw(G)
# plt.show()

# Set Font for chinese display
plt.rcParams['font.sans-serif']=['SimHei']
plt.rcParams['axes.unicode_minus']=False

G = nx.DiGraph()

df = pd.read_csv(R'C:\Users\13427\Desktop\code\linux-tools\python\GNN\data\relation_raw.csv')

nodes1 = [node for node in zip(df['人物1'],df['家族1'])]
nodes2 = [node for node in zip(df['人物2'],df['家族2'])]

nodes = set(nodes1) | set(nodes2)
nodes = list(nodes)

nodes = [(node[0],{'kin':node[1]}) for node in nodes]

edges = [(edge[0],edge[1],{'relation':edge[2]})for edge in zip(df['人物1'],df['人物2'],df['关系'])]
G.add_nodes_from(nodes)
G.add_edges_from(edges)

pos = nx.spring_layout(G,seed=10)
pos = nx.shell_layout(G)

plt.figure(figsize=(15,15))
edge_labels = nx.get_edge_attributes(G, "relation")
nx.draw_networkx_edge_labels(G, pos, edge_labels)
nx.draw(G,pos=pos,with_labels=True)

# plt.show()
# print(G)
# G.nodes(data=True)

# Inspect degree
degree = sort_utils.sort_tuple_list_by_index(nx.degree(G),1)
print(degree)

# Inspect the pagerank
pagerank = sort_utils.sort_dict_by_value(nx.pagerank(G,alpha=0.75))
print(pagerank)

# Inspect the eigenvalue-centrality  
print(sort_utils.sort_dict_by_value(nx.eigenvector_centrality(G)))

