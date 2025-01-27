import re

class Vertex:
    def __init__(self, data, label):
        self.data = data
        self.label = label
    
    def __str__(self):
        return "(data: {} label: {})".format(self.data, self.label)

class Edge:
    def __init__(self, data, from_vertex, to_vertex, label):
        self.data = data
        self.from_vertex = from_vertex
        self.to_vertex = to_vertex
        self.label = label
    
    def __str__(self):
        return "<data: {} from: {} to: {} label: {}>".format(self.data, self.from_vertex.data, self.to_vertex.data, self.label)

class Graph:
    def __init__(self):
        self._vertices = {}
        self._outgoing = {}
        self._incoming = {}
        self._fromto = {}
    
    def insert_vertex(self, data) -> Vertex:
        if data in self._vertices.keys():
            raise Exception("duplicate vertex")

        v = Vertex(data, None)
        self._vertices[data] = v
        self._outgoing[v] = set()
        self._incoming[v] = set()
        self._fromto[v] = set()

        return v
    
    def insert_edge(self, data, from_vertex: Vertex, to_vertex: Vertex) -> None:
        if to_vertex in self._fromto[from_vertex]:
            raise Exception("duplicate edge")

        e = Edge(data, from_vertex, to_vertex, None)
        self._outgoing[from_vertex].add(e)
        self._incoming[to_vertex].add(e)
        self._fromto[from_vertex].add(to_vertex)

    def vertices(self):
        return self._vertices.values()

    def outgoing(self, v: Vertex):
        return self._outgoing[v]

    def incoming(self, v: Vertex):
        return self._incoming[v]

    def label_vertex(self, v: Vertex, label) -> None:
        v.label = label

    def label_edge(self, e: Edge, label) -> None:
        e.label = label

    def has(self, data) -> bool:
        try:
            self._vertices[data]
            return True
        except KeyError:
            return False

    def get(self, data) -> Vertex:
        return self._vertices[data]
        


    def __str__(self):
        s = ""
        for k,v in self._outgoing.items():
            s += "{} -> ".format(k)
            for elem in v:
                s += "{}, ".format(elem)
            s += "\n"
        return s

    def breadth_first_search(self, start) -> dict:
        distances = {v:0 for v in self.vertices()}
        explored = {v:False for v in self.vertices()}
        previous = {v:None for v in self.vertices()}

        queue = [start]
        distances[start] = 0
        explored[start] = True

        while len(queue) > 0:
            v = queue.pop()
            for e in self.outgoing(v):
                u = e.to_vertex
                if not explored[u]:
                    distances[u] = distances[v] + 1
                    explored[u] = True
                    previous[u] = v
                    queue.insert(0, u)

        return distances

class ValveSim():
    def __init__(self, filename: str, start = "AA"):
        self.start = start
        self.graph = self.parse(filename)
        self.sparse_graph = self.parse_sparse()

    def parse(self, filename: str) -> Graph:
        graph = Graph()
        valve_exp = re.compile(r"[A-Z][A-Z]")
        flow_exp = re.compile(r"\d+")

        with open(filename) as file:
            for i, line in enumerate(file):
                valve_match = valve_exp.findall(line)
                flow_match = flow_exp.findall(line)

                if len(valve_match) > 0 and len(flow_match) > 0:

                    # check if vertex with that data already exists; if not insert
                    v = graph.get(valve_match[0]) if graph.has(valve_match[0]) else graph.insert_vertex(valve_match[0])
                    
                    graph.label_vertex(v, int(flow_match[0]))

                    # connect neighbor vertices
                    for i in range(1, len(valve_match)):
                        u = graph.get(valve_match[i]) if graph.has(valve_match[i]) else graph.insert_vertex(valve_match[i])
                        graph.insert_edge(None, v, u)

        return graph

    def parse_sparse(self):
        sparse_graph = Graph()        

        for v in self.graph.vertices():
            # non-zero pump rate
            if v.label > 0:
                sv = sparse_graph.insert_vertex(v.data)
                sparse_graph.label_vertex(sv, v.label)

        for v in sparse_graph.vertices():
            search = self.graph.get(v.data)
            distances = self.graph.breadth_first_search(search)
            for u in sparse_graph.vertices():
                if u is not v:
                    search = self.graph.get(u.data)
                    sparse_graph.insert_edge(distances[search], v, u)
        
        # finally insert start, which the other verticies should not point back to
        v = self.graph.get(self.start)
        distances = self.graph.breadth_first_search(v)

        v = sparse_graph.insert_vertex(self.start)

        for u in sparse_graph.vertices():
            if u is not v:
                search = self.graph.get(u.data)
                sparse_graph.insert_edge(distances[search], v, u)
        
        return sparse_graph

    def simulate(self, time:int = 30) -> int:
        
        v = self.sparse_graph.get(self.start)

        explored = {u : False for u in self.sparse_graph.vertices()}

        return self._simulate(v, time, explored)

    def _simulate(self, source: Vertex, time:int, explored: dict) -> int:
        max_pump = 0
        for e in self.sparse_graph.outgoing(source):
            u = e.to_vertex

            # time to travel + time to turn on valve
            elapsed = e.data + 1
            # only visit if valve not on
            if not explored[u] and elapsed <= time:
                
                # set valve to explored for deeper recursions
                explored[u] = True

                rem = time - elapsed

                # save the max valve from this subtree of the recursion
                pump = u.label * rem + self._simulate(u, rem, explored)

                if pump > max_pump:
                    max_pump = pump

                # after exploring subtree, reset the explored status
                explored[u] = False
        return max_pump

    def simulate_p2(self, time:int = 26) -> int:
        
        v = self.sparse_graph.get(self.start)

        explored = {u : False for u in self.sparse_graph.vertices()}

        return self._simulate_p2(v, v, time, explored)

    def _simulate_p2(self, source: Vertex, source2: Vertex, transit1, transit2, time:int, explored: dict) -> int:
        max_pump = 0

        # analyze each possible route for the two players
        for e in self.sparse_graph.outgoing(source):
            for e2 in self.sparse_graph.outgoing(source2):
                u = e.to_vertex
                u2 = e2.to_vertex

                # time to travel + time to turn on valve
                elapsed = e.data + 1

                # only visit if valve not on
                if not explored[u] and elapsed <= time:
                    
                    # set valve to explored for deeper recursions
                    explored[u] = True

                    rem = time - elapsed

                    # save the max valve from this subtree of the recursion
                    pump = u.label * rem + self._simulate_p2(u, rem, explored)

                    if pump > max_pump:
                        max_pump = pump

                    # after exploring subtree, reset the explored status
                    explored[u] = False

        return max_pump

            
            
            


if __name__ == "__main__":

    sim = ValveSim("input16.txt")

    print(sim.simulate())

    
