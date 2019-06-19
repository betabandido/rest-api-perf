#! /usr/bin/env Rscript

library(data.table)
library(ggplot2)

args = commandArgs(trailingOnly = TRUE)

if (length(args) != 1) {
  stop('USAGE: make-plot.R <implementation>')
}

implementation <- args[1]

dt <- fread(sprintf('results-%s.csv', implementation))

pl <- ggplot(dt, aes(`requested-qps`, `actual-qps`)) +
  geom_line() +
  geom_point() +
  xlab('Requested QPS') +
  ylab('Actual QPS') +
  ylim(0, 32000)
ggsave(sprintf('qps-%s.png', implementation), 
       pl, width = 5, height = 4, units = "in")

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
ggsave(sprintf('latency-%s.png', implementation),
       pl, width = 5, height = 4, units = "in")
