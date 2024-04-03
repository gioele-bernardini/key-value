#!/bin/bash

# From https://ioflood.com/blog/bash-regex/

# You can use regular expressions in Bash with the =~ operator in an if statement. 
# The syntax for this would be as follows: if [[ 'Hello World' =~ Hello ]]. 
# This operator allows you to match a string against a regular expression right within your Bash script.

if [[ 'Hello World' =~ Hello ]]; then echo 'Match found'; else echo 'Match not found'; fi

# Output:
# 'Match found'

# --------------------------------------------------------------------------------------------------

string='Bash regex is powerful'
pattern='powerful'

if [[ $string =~ $pattern ]]; then echo 'Match found'; else echo 'Match not found'; fi

# Output:
# 'Match found'

# --------------------------------------------------------------------------------------------------
# Character classes allow you to match any character from a specific set. 
# For instance, [a-z] matches any lowercase letter, while [0-9] matches any digit.

string='My number is 12345'
pattern='[0-9]+'

if [[ $string =~ $pattern ]]; then echo 'Match found'; else echo 'Match not found'; fi

# Output:
# 'Match found'

# --------------------------------------------------------------------------------------------------
# Capture groups are a way to treat multiple characters as a single unit. 
# They are created by placing the characters to be grouped inside a set of parentheses (). 
# For example, (abc) is a capture group that matches the exact sequence of ‘abc’

string='Bash regex is aaaamazing'
pattern='a{3}'

if [[ $string =~ $pattern ]]; then echo 'Match found'; else echo 'Match not found'; fi

# Output:
# 'Match found'

# --------------------------------------------------------------------------------------------------
# Grep is a command-line utility used for searching plain-text data for lines that match a regular expression. 
# It’s a powerful tool for regex in Bash.

echo 'Bash regex is powerful' | grep -E 'powerful'

# Output:
# 'Bash regex is powerful'


# --------------------------------------------------------------------------------------------------
# Whitespace characters like spaces and tabs can sometimes cause unexpected behavior in regex. 
# One way to handle this is by using the [:space:] character class, which matches any whitespace character.

string='Bash regex is    powerful'
pattern='Bash regex is[[:space:]]+powerful'

if [[ $string =~ $pattern ]]; then echo 'Match found'; else echo 'Match not found'; fi

# Output:
# 'Match found'


# --------------------------------------------------------------------------------------------------
# Special characters such as *, ., ?, and others have special meanings in regex. 
# If you want to match these characters literally, you need to escape them using a backslash \.

string='Bash regex is *powerful*'
pattern='\*powerful\*'

if [[ $string =~ $pattern ]]; then echo 'Match found'; else echo 'Match not found'; fi

# Output:
# 'Match found'


