from typing import List
# hashtable(use set which implement that)
# Psuedocode
# for email in emails
#  - parse local name from it
#  - add actual sent email to set
# return length of set
# Complexity
# - time complexity: n * (O(1) of parsing + O(1) of lookup)
# - space complexity: O(n) of storing returning elements

import re

class Solution:
    def numUniqueEmails(self, emails: List[str]) -> int:
        actual_emails_to_send = set()
        pattern = r'(?P<localname>[^\+]+)\+?.*(?P<domain>@.+)'
        for email in emails:
            m = re.match(pattern, email)
            actual_emails_to_send.add(m.group('localname').replace('.', '') + m.group('domain'))
        return len(actual_emails_to_send)
