"""
Definition for singly-linked list.

class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None
"""


class Solution(object):
    """Solution for 141."""

    def hasCycle(self, head):
        """Check whether there is a cycle."""
        if head is None:
            return False

        h1 = head.next
        h2 = head

        while h1 is not None and h1.next is not None:
            if h1 == h2:
                return True
            h1 = h1.next.next
            h2 = h2.next

        return False
