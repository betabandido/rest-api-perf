library(data.table)
library(ggplot2)

dt <- fread('results.csv')

pl <- ggplot(dt, aes(`requested-qps`, `actual-qps`)) +
  geom_line() +
  xlab('Requested QPS') +
  ylab('Actual QPS')
print(pl)

dt[, c('actual-qps', 'total-count', 'success-count') := NULL]
dt <- melt(dt, 
           id.vars=c('requested-qps'), 
           variable.name = 'metric', 
           value.name = 'latency')

pl <- ggplot(dt, aes(`requested-qps`, latency, color = metric)) +
  geom_line() +
  xlab('Requested QPS') +
  ylab('Latency (ms)') +
  theme(legend.position = "bottom")
print(pl)
