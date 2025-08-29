# 9. Palindrome Number
IsPalindrome ← |1 ≍ ⇌. °⋕
IsPalindrome 121
IsPalindrome ¯121
IsPalindrome 10
IsPalindrome 3223

# 66. Plus One

PlusOne ← |1 (
  ⊂ 0     # Prefix a zero
  ⌕ 9 .   # Duplicate and find the 9s
  ⇌ \↧ ⇌  # Drop everything after the first 0
  ↘₁ ˜⊂ 1 # Append an extra 1
  ◿ 10 +  # Add the ones and modulo 10
  ⍣°(⊂0)  # Remove a prefix 0 if there is one
)

PlusOne [1 2 9]
PlusOne [1 2 1]
PlusOne [9 9 9]
PlusOne [9 2 9]
