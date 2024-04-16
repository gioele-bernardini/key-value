class Solution:
    def isValid(self, s: str) -> bool:
        open_brackets = 0

        curly = 0
        square = 0
        round = 0

        for c in s:
            if c == '{':
                open_brackets += 1
                curly = open_brackets
            if c == '[':
                open_brackets += 1
                square = open_brackets
            if c == '(':
                open_brackets += 1
                round = open_brackets

            if c == '}':
                if curly == 0 or curly < open_brackets:
                    return False
                else:
                    curly -= 1
                    open_brackets -= 1
            if c == ']':
                if square == 0 or square < open_brackets:
                    return False
                else:
                    square -= 1
                    open_brackets -= 1
            if c == ')':
                if round == 0 or round < open_brackets:
                    return False
                else:
                    round -= 1
                    open_brackets -= 1

        if open_brackets != 0:
            return False   

        return True