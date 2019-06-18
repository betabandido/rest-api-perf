import argparse
import json
import sys

COLUMNS = [
    'requested-qps',
    'actual-qps',
    'latency-50',
    'latency-75',
    'latency-90',
    'latency-99',
    'latency-99.9',
    'total-count',
    'success-count'
]

parser = argparse.ArgumentParser()
parser.add_argument('--file', metavar='FILE')
parser.add_argument('--qps', metavar='QPS', type=int)
parser.add_argument('--print-headers', action='store_true')

args = parser.parse_args()

if args.print_headers:
    print(','.join(COLUMNS))
    sys.exit()

with open(args.file, 'r') as f:
    result = json.load(f)

    if int(result['RequestedQPS']) != args.qps:
        raise 'QPS does not match'

    duration = dict([
        (e['Percentile'], e['Value'] * 1000) 
        for e in result['DurationHistogram']['Percentiles']
    ])
    
    values = [
        result['RequestedQPS'],
        result['ActualQPS'],
        duration[50],
        duration[75],
        duration[90],
        duration[99],
        duration[99.9],
        result['Sizes']['Count'],
        result['RetCodes']['200']
    ]
    print(','.join([str(e) for e in values]))
