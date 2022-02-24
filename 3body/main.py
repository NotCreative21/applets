import rebound
import numpy as np
import matplotlib.pyplot as plt

sim = rebound.Simulation()
sim.units = ['mearth', 'day', 'AU']

sim.add(m=40000)

sim.add(m=.25, P=5, e=.04)
sim.add(m=1.6, P=11)

rebound.OrbitPlot(sim)

#sim.integrate(100)

x_pos = np.empty((3, 10))
y_pos = np.empty((3, 10))

times = np.linspace(0, 100, num=10)

for i, t in enumerate(times):
    sim.integrate(t)
    x_pos[0, i] = sim.particles[0].x
    y_pos[0, i] = sim.particles[0].y
    x_pos[1, i] = sim.particles[1].x
    y_pos[1, i] = sim.particles[1].y
    x_pos[2, i] = sim.particles[2].x
    y_pos[2, i] = sim.particles[2].y

plt.show()
