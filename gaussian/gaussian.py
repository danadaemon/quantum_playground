import pennylane as qp
import jaxopt
from jax import numpy as np


dev_gaussian = qp.device("default.gaussian", wires=1)


@qp.qnode(dev_gaussian)
def mean_photon_gaussian(mag_alpha, phase_alpha, phi):
    qp.Displacement(mag_alpha, phase_alpha, 0)
    qp.Rotation(phi, 0)
    return qp.expval(qp.NumberOperator(0))


def cost(params):
    return (mean_photon_gaussian(params[0], params[1], params[2]) - 1.0) ** 2


init_params = np.array([0.015, 0.02, 0.005])
print(cost(init_params))

opt = jaxopt.GradientDescent(cost, stepsize=0.1, acceleration=False)
steps = 20
params = init_params
opt_state = opt.init_state(params)

for i in range(steps):
    params, opt_state = opt.update(params, opt_state)
    print("Cost after step {:5d}: {:8f}".format(i + 1, cost(params)))

print("Optimized mag_alpha:{:8f}".format(params[0]))
print("Optimized phase_alpha:{:8f}".format(params[1]))
print("Optimized phi:{:8f}".format(params[2]))
