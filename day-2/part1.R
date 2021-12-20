## mydata <- read.table("input", col.names <- c("Direction", "Distance"))
mydata <- read.table("input")
mydata
horiz <- mydata[mydata$V1 == "forward", 2]
horiz
posDepth <- mydata[mydata$V1 == "down", 2]
posDepth
negDepth <- mydata[mydata$V1 == "up", 2]
negDepth
print(sum(horiz) * (sum(posDepth) - sum(negDepth)))
