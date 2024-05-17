# 4 Solution

### Quiz
*What is the version number indicated by the following 4 bytes little endian in hexadecimal format: `10000000`?*

### Solution
Remember, the least significant byte appears first. Let's reverse it so we can do the base math we're familiar with:
`00 00 00 10`

And here's the math:
`0*16^7 + 0*16^6 + 0*16^5 + 0*16^4 + 0*16^3 + 0*16^2 + 1*16^1 + 0*16^0`

So, the version number is `16`! Maybe one day Bitcoin transactions will get there!
