import os

test_dir = "test"
dir_list = [
    "alpha/beta/gamma3",
    "alpha/beta/delta6",
    "alpha/beta/epsil@on8",
    "alpha/betabeta/epsil0n/zeta1",
    "alpha/betabeta/epsil#on/eta4",
    "alpha/betabetaa/thet@a/iota7",
    "alpha/alpha/thet#a/iota0/kappa1",
    "alpha/alpha/beta/theta3/lambda4",
    "beta/mu6/n@u7",
    "beta/mu9/xi0",
    "beta/mu2/omicron3/pi4",
    "beta/mu6/rho7",
    "gamma/sigma9/t@u0",
    "gamma/sigm#a/upsilon3",
    "gamma/ph!i/chi6",
    "gamma/ph!i/psi9/omega0",
    "gamma/alpha2",
    "gamma/alpha4/beta5",
    "delta/gamma7",
    "delta/delta9/epsilon0",
    "delta/del@ta/zeta2/eta3",
    "delta/thet#a/iota5",
    "delta/kapp@a/lambda7",
    "delta/mu9",
    "epsilon/nu1/x2",
    "epsilon/xi4",
    "epsilon/omicron6/pi7",
    "epsilon/omicron9/rh0",
    "epsilon/sigm#a/tau2",
    "epsilon/upsil@on/phi4",
    "epsilon/beta/chi6/ps!i7",
    "epsilon/beta/omega9/alpha0",
    "epsilon/beta/beta2/gamma3",
    "epsilon/beta/del@ta/epsilon5",
    "epsilon/beta/eta7/theta8",
    "epsilon/beta/iot@a/kappa0",
]

for dir in dir_list:
    path = os.path.join(test_dir, dir)
    os.makedirs(path, exist_ok=True)

print("Directories created successfully.")

