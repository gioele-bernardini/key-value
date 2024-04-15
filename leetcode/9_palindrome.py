class Solution:
  def isPalindrome(x: int) -> bool:
    return str(x) == str(x)[::-1]

if __name__ == '__main__':
  solution = Solution
  out = solution.isPalindrome(13)
  print(out)
