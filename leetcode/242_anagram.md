# Idea: Contiamo le lettere, una sorta di dizionario insomma!
# Stesse lettere, stessa quantita'! Usiamo quindi una hashmap.
# O(s + t); s + t complessità, memoria.

class Solution:
	def isAnagram(self, s: str, t: str) -> bool:
		if len(s) != len(t):
			return False

		countS, countT = {}, {}

		for i in range(len(s)):
      # countS[s[i]] = 1 + countS[s[i]] # rise a key error!
			countS[s[i]] = 1 + countS.get(s[i], 0)
			countT[t[i]] = 1 + countT.get(t[i], 0)
		for key in countS:
			if countS[key] != countT.get(key, 0): # same here!
				return False

		return True

# Come risolvo il problema con O(1) di memoria?
# Intuizione: posso ordinare le stringhe!
# Restituiamo True se le stringhe ordinate sono uguali, altrimenti False.

return sorted(s) == sorted(t)
