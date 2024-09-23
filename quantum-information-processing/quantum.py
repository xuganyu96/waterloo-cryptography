from sympy import Matrix, sqrt, ShapeError
from sympy.physics.quantum import TensorProduct

PAULI_X = Matrix([
    [0, 1],
    [1, 0]
])
PAULI_Z = Matrix([
    [1, 0],
    [0, -1]
])
HADAMARD = Matrix([
    [1/sqrt(2), 1/sqrt(2)],
    [1/sqrt(2), -1/sqrt(2)]
])

def controlled_u(u: Matrix):
    """Given unitary operator, compute the unitary operator on controlled-U gate
    -----*-----
         |
    ----|U|----
    """
    if u.shape != (2, 2):
        raise ShapeError("U must be unitary oeprator on a single qubit")
    return Matrix([
        [Matrix.eye(2), Matrix.zeros(2)],
        [Matrix.zeros(2), u]
    ])

def inverted_controlled_u(u: Matrix):
    """Given unitary operator, compute the inverted unitary operator
    ---|U|---
        |
    ----*----
    """
    if u.shape != (2, 2):
        raise ShapeError("U must be unitary operator on a single qubit")
    return Matrix([
        [1, 0, 0, 0],
        [0, u[0,0], 0, u[0,1]],
        [0, 0, 1, 0],
        [0, u[1,0], 0, u[1,1]],
    ])