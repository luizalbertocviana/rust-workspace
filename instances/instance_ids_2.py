from typing import List

# for generator6 generator 7 generator8 generator9 generator10
# generator16 generator17 generator18 generator19 generator20

def get_instance_ids() -> List[str]:
    return ["%d_%.2f_%.2f-%d" % (n, d, dag_d, i)
            for n in [30, 60, 90, 120]
            for d in [0.25, 0.50, 0.75]
            for dag_d in [0.25, 0.50, 0.75]
            for i in range(5)]
