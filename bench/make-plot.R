#! /usr/bin/env Rscript

library(data.table)
library(ggplot2)

args = commandArgs(trailingOnly = TRUE)

if (length(args) != 2) {
  stop('USAGE: make-plot.R <implementation> <keepalive>')
}

implementation <- args[1]
keepalive <- args[2]

dt <- fread(sprintf('results-%s-keepalive:%s.csv', implementation, keepalive))

# QPS plot

pl <- ggplot(dt, aes(`requested-qps`, `actual-qps`)) +
  geom_line() +
  geom_point() +
  xlab('Requested QPS') +
  ylab('Actual QPS') +
  ylim(0, max(dt[, `requested-qps`]))
ggsave(sprintf('qps-%s-keepalive:%s.png', implementation, keepalive), 
       pl, width = 5, height = 4, units = "in")

# latency plot

dt[, c('actual-qps', 'total-count', 'success-count') := NULL]
dt <- melt(dt, 
           id.vars=c('requested-qps'), 
           variable.name = 'metric', 
           value.name = 'latency')

pl <- ggplot(dt, aes(`requested-qps`, latency, color = metric)) +
  geom_line() +
  geom_point() +
  xlab('Requested QPS') +
  ylab('Latency (ms)') +
  ylim(0, NA)
ggsave(sprintf('latency-%s-keepalive:%s.png', implementation, keepalive),
       pl, width = 5, height = 4, units = "in")

# latency plot (w/o 99th percentile)

dt <- dt[metric != 'latency-99.9']

pl <- ggplot(dt, aes(`requested-qps`, latency, color = metric)) +
  geom_line() +
  geom_point() +
  xlab('Requested QPS') +
  ylab('Latency (ms)') +
  ylim(0, NA)
ggsave(sprintf('zoom-latency-%s-keepalive:%s.png', implementation, keepalive),
       pl, width = 5, height = 4, units = "in")
