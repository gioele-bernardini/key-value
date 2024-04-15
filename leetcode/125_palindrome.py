class Solution:
  def isPalindrome(self, s: str) -> bool:
    out = ''.join(c.lower() for c in s if c.isalnum())
    return out == out[::-1]
  
  def isPalindrome2(self, s: str) -> bool:
        

if __name__ == '__main__':
  s = 'A man, a plan, a canal: Panama'
  solution = Solution()
  print(solution.isPalindrome(s))