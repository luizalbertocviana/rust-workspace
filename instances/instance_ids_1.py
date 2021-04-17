from typing import List

# for generator1 genertor2 generator3 generator4 generator5
# generator11 generator12 generator13 generator14 generator15

def get_instance_ids() -> List[str]:
    return ["%d_%.2f_%d-%d" % (n, d, b, i)
            for n in [30, 60, 90, 120, 150]
            for d in [0.25, 0.50, 0.75]
            for b in [2, 4, 8]
            for i in range(5)]
