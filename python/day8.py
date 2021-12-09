CHARS = ['a', 'b', 'c', 'd', 'e', 'f', 'g']

def parse_pattern(w):
    # represent e.g. the word acf as the tuple
    # (1,0,1,0,0,1,0)
    # (a,b,c,d,e,f,g)
    # this representation will be called binary pattern in what follows

    res = [0] * len(CHARS)
    for i, c in enumerate(CHARS):
        if c in w:
            res[i] = 1

    return tuple(res)

def permute(pi, bin_pattern):
    res = [0] * len(CHARS)

    for i, b in enumerate(bin_pattern):
        res[pi[i]] = b

    return tuple(res)

def solve():
    DIGITS = [
        'abcefg', #0
        'cf', #1
        'acdeg', #2
        'acdfg', #3
        'bcdf', #4
        'abdfg', #5
        'abdefg', #6
        'acf', #7
        'abcdefg', #8
        'abcdfg' #9
    ]

    with open("problems/problem8") as f:
        lines = f.readlines()

    observations = []
    for line in lines:
        i, o = line.split(' | ')

        i = [parse_pattern(w) for w in i.split()]
        o = [parse_pattern(w) for w in o.split()]

        observations.append((i, o))

    # This is not the most efficient solution for part 1 but it does not really matter
    prob1 = 0
    target_lengths = {len(DIGITS[1]),len(DIGITS[4]), len(DIGITS[7]), len(DIGITS[8])}
    for _, output in observations:
        for pattern in output:
            ones = sum(pattern)
            if  ones in target_lengths:
                prob1 += 1

    print("Problem 1:", prob1)

    ### Part 2

    # Now, our task for part 2 can be stated like this: Find a permutation pi of {0, 1,...,6} such that
    # for every binary input pattern (x0, ..., x6) the binary pattern (x)^pi = (x_pi^-1(0), x_pi^-1(1), ..., x_pi^-1(6))
    # is a valid binary pattern, i.e. a key in bin_pattern_to_digit.
    # Note that it is somewhat more natural to work with the inverse of the permutation in this instance:
    # If pi(i) = j then the jth component of the permuted tuple equals x_pi^-1{j} = x_i

    bin_pattern_to_digit = {parse_pattern(w):i for i, w in enumerate(DIGITS)}

    # We can also reduce the search space using a simple observation: 
    # Suppose that (x0, ..., x6) is an input pattern with exactly 3 one entries.
    # Then it has to represent a 7 and it holds that
    # (x)^pi = (x_pi^-1(0), ..., x_pipi^-1(6)) = (1, 0, 1, 0, 0, 1, 0)
    # If x_j = 1 then the pi(j) th component of the permuted vector is also 1, so that
    # pi(j) \in {0, 2, 5} holds.
    possible_positions = {i:set() for i in range(len(CHARS) + 1)}
    for bin_pattern in bin_pattern_to_digit:
        ones = sum(bin_pattern)
        one_indices = (i for i, x in enumerate(bin_pattern) if x == 1)

        possible_positions[ones].update(one_indices)
    
    prob2 = 0
    for inp, outp in observations:
        possible_pi = {i:set(range(len(CHARS))) for i in range(len(CHARS))}

        for bin_pattern in inp:
            l = sum(bin_pattern)

            for i, b in enumerate(bin_pattern):
                if not b:
                    continue

                possible_pi[i] &= possible_positions[l]

        # We find this permutation using backtracking
        def make_pi(pi, i, used):
            if i == len(CHARS):
                if len(used) == len(CHARS):
                    # pi is an actual permutation! Now verify its what we are looking for.
                    for w in inp:
                        if permute(pi, w) not in bin_pattern_to_digit:
                            return None

                    return pi
                else:
                    return None

            for j in possible_pi[i]:
                if j in used:
                    continue

                used.add(j)
                pi[i] = j
                
                res = make_pi(pi, i + 1, used)

                if res:
                    return res

                pi[i] = None
                used.remove(j)

        pi = make_pi({i:None for i in range(len(CHARS))}, 0, set())

        displayed_number = 0
        for w in outp:
            displayed_number *= 10
            displayed_number += bin_pattern_to_digit[permute(pi, w)]

        prob2 += displayed_number

    print("Problem 2:", prob2)

if __name__ == '__main__':
    solve()