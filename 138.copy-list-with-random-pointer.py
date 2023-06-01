"""
# Definition for a Node.

class Node:
    def __init__(self, x, next=None, random=None):
        self.val = int(x)
        self.next = next
        self.random = random
"""


class Solution(object):
    """Solution for leetcode problem 138."""

    def __init__(self):
        """Init function."""
        self.d = {}

    def getNode(self, n):
        """Get new node, or node from dict."""
        if n:
            if n in self.d:
                return self.d[n]
            else:
                self.d[n] = Node(n.val, None, None)
                return self.d[n]
        return None

    def copyRandomList(self, head):
        """Actually copy all of the values."""
        if not head:
            return head

        h1 = head
        h2 = Node(h1.val, None, None)
        self.d[h1] = h2
        while h1 is not None:
            h2.random = self.getNode(h1.random)
            h2.next = self.getNode(h1.next)

            h1 = h1.next
            h2 = h2.next
        return self.d[head]
