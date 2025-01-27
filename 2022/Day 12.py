class Vertex:
    def __init__(self, data, label):
        self.data = data
        self.label = label
    
    def __str__(self):
        return "(data: {} label: {})".format(self.data, self.label)

class Graph:
    def __init__(self):
        self.connections = {}
        self.source = None
        self.lowest = set()
        self.end = None
    
    def insert(self, v: Vertex) -> None:
        self.connections[v] = set()
        if v.label == "S":
            self.source = v
            self.lowest.add(v)
        elif v.label == "a":
            self.lowest.add(v)
        elif v.label == "E":
            self.end = v
    
    def connect(self, v: Vertex, neighbor: Vertex) -> None:
        if v not in self.connections.keys() or neighbor not in self.connections.keys():
            raise Exception("invalid nodes")

        # reencode
        v_label = "a" if v.label == "S" else "z" if v.label == "E" else v.label
        neighbor_label = "a" if neighbor.label == "S" else "z" if neighbor.label == "E" else neighbor.label
        if ord(neighbor_label) - ord(v_label) <= 1:
            self.connections[v].add(neighbor)

    def vertices(self):
        return self.connections.keys()
    
    def neighbors(self, v: Vertex):
        return self.connections[v]

    def __str__(self):
        s = ""
        for k,v in self.connections.items():
            s += "{} -> ".format(k)
            for elem in v:
                s += "{}, ".format(elem)
            s += "\n"
        return s

def breadth_first_search(graph: Graph, start, end):
    distances = {v:0 for v in graph.vertices()}
    explored = {v:False for v in graph.vertices()}
    previous = {v:None for v in graph.vertices()}

    queue = [start]
    distances[start] = 0
    explored[start] = True

    while len(queue) > 0:
        v = queue.pop()
        for u in graph.neighbors(v):
            if not explored[u]:
                distances[u] = distances[v] + 1
                explored[u] = True
                previous[u] = v
                queue.insert(0, u)

    # cur = end
    # while cur is not None:
    #     print(cur)
    #     cur = previous[cur]

    return distances[end]

def parse(filename: str):
    graph = Graph()
    vertices = []
    with open(filename) as file:
        
        lines = [line.strip("\n") for line in file.readlines()]

        for i, line in enumerate(lines):
            vert_row = []
            for j, c in enumerate(line):
                    vertex = Vertex((i, j), c)

                    graph.insert(vertex)

                    vert_row.append(vertex)
            
            vertices.append(vert_row)
        
        for i, row in enumerate(vertices):
            for j, v in enumerate(row):
                if i > 0:
                    graph.connect(v, vertices[i-1][j])
                if i < len(vertices) - 1:
                    graph.connect(v, vertices[i+1][j])
                if j > 0:
                    graph.connect(v, vertices[i][j-1])
                if j < len(row) - 1:
                    graph.connect(v, vertices[i][j+1])
    return graph

if __name__ == "__main__":

    graph = parse("input12.txt")

    print(graph)

    print(breadth_first_search(graph, graph.source, graph.end))
    print(len(graph.lowest))
    res = [breadth_first_search(graph, start, graph.end) for start in graph.lowest if breadth_first_search(graph, start, graph.end) != 0]

    print(min(res))
    
