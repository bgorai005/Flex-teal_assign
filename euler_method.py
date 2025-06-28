import numpy as np
import matplotlib.pyplot as plt

# funtion defination
def f(t,y):
    return np.cos(t)-y

#initial condition 
t0=0
y0=1

#number of iteration and step size
N=int(input('Enter the number of iteration:= '))
tend=5
h=(tend-t0)/N  # step size


# intialize the arrays
t=np.zeros(N+1)
y=np.zeros(N+1)

#intial condtion
t[0]=0
y[0]=1

#euler approximation formaula
for i in range(N):
    t[i+1]=t[i]+h
    y[i+1]=y[i]+h*f(t[i],y[i])


#exact solution
def exact_sol(t):
    return 0.5*(np.cos(t)+np.sin(t)+np.exp(-t))

#exact values
y_exact=np.zeros(N+1)

for i in range(N+1):
    y_exact[i]=exact_sol(t[i])


#plot the image 
plt.plot(t,y,label='Eular approximation method')
plt.plot(t,y_exact,label='Exact solution')
plt.xlabel('t')
plt.ylabel('y(t)')
plt.title("Euler's Method: dy/dt = cost-y")
plt.legend()
plt.grid()
plt.show()



