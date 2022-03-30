#!/usr/bin/env  python3

import io
import json
import sys

from collections import Counter

#sys.stdin.reconfigure(encoding='utf-8')

def method0():
    lines = map(str.strip, io.TextIOWrapper(sys.stdin.buffer, encoding='utf-8'))
    return
    #counts = Counter(( yield from iter(l.split()) for l in lines ))
    #counts = Counter(( yield from l.split() for l in lines ))

    return counts



def method1():
    lines = map(str.strip, io.TextIOWrapper(sys.stdin.buffer, encoding='utf-8'))
    words = ( word for line in lines for word in line.split() )
    counts = Counter(words)

    return counts






if __name__ == '__main__':
    counts = method1()
    #print(json.dumps(counts, ensure_ascii=False))
    #print('\n'.join(f'{k} {v}' for k, v in counts.items()))
    print('\n'.join(f'{k} {v}' for k, v in counts.most_common(10)))
