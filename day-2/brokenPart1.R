myData = read.delim("input", header = FALSE)
depth = 0
horiz = 0
for (x in myData) {
  dist = as.numeric(substring(x, nchar(x) - 1, nchar(x)))
  if (grepl("up", x)) {
    depth = depth + (dist * -1)
  } else if (grepl("down", x)) {
    depth = depth + dist
  } else if (grepl("forward", x)) {
    horiz = horiz + dist
  }
}
print(depth)
print(horiz)
print(sum(depth) * sum(horiz))
