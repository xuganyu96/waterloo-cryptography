# QIC 710: quantum information processing (Fall 2024)
[course website](https://cleve.iqc.uwaterloo.ca/qic710/)

# n-qubit systems

## Controlled gates
Consider a 2-qubit system $\vert \psi \rangle \in \mathbb{C}^{2^2}$. A controlled $U$ gate is such that:
- If the first qubit is 0, then the second qubit remains the same
- If the first qubit is 1, then $U$ is applied to the second qubit

Controlled-U gate is a unitary operation, the corresponding matrix is:

$$
V = \begin{bmatrix}
I & 0 \\
0 & U
\end{bmatrix}
$$

```
Controlled-U gate:
----------*---------
          |
---------|U|--------
```

We can flip the control bits: apply U to the first qubit if and only if the second qubit is 1.

```
Inverted Controlled-U gate
-----|U|-----
      |
------*------
```

The corresponding matrix can be computed using a linearity argument: 
- $\vert 00 \rangle \mapsto \vert 00 \rangle$
- $\vert 01 \rangle \mapsto U\vert 0 \rangle \otimes \vert 1 \rangle$
- $\vert 10 \rangle \mapsto \vert 10 \rangle$
- $\vert 11 \rangle \mapsto U\vert 1 \rangle \otimes \vert 0 \rangle$

The resulting matrix is 

$$
V = \begin{bmatrix}
1 & 0 & 0 & 0\\
0 & u_{00} & 0 & u_{01}\\
0 & 0 & 1 & 0\\
0 & u_{10} & 0 & u_{11}\\
\end{bmatrix}
$$

Recall the NOT gate:

$$
X = \begin{bmatrix}
0 & 1 \\
1 & 0
\end{bmatrix},
X\vert 0 \rangle = \vert 1 \rangle, X\vert 1 \rangle = \vert 0 \rangle
$$

A Controlled-NOT (CNOT) gate is analogous the XOR gate in classical bits $\texttt{CNOT}(\vert \alpha \rangle \vert \beta \rangle) = \vert \alpha\rangle \vert\alpha\oplus\beta\rangle$ for $\alpha, \beta \in \{0,1\}$.

**CNOT might change the state of the control bit**:

$$
\texttt{CNOT}(\vert + \rangle \vert - \rangle) = \vert - \rangle \vert - \rangle
$$