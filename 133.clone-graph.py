"""
# Definition for a Node.

class Node(object):
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""


class Solution(object):
    """Solution to problem 133."""

    def __init__(self):
        """Init Function."""
        self.visited = {}

    def cloneGraph(self, node):
        """
        Clone Graph function.

        :type node: Node
        :rtype: Node
        """
        if not node:
            return node

        if node in self.visited:
            return self.visited[node]

        cloned = Node(node.val, [])  # noqa: F821

        self.visited[node] = cloned

        if node.neighbors:
            cloned.neighbors = [self.cloneGraph(n) for n in node.neighbors]

        return cloned
