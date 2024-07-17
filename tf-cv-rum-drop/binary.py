import tensorflow as tfk
import matplotlib.pyplot as pl
import numpy as np

def tp(x,l):
    pl.figure(figsize=(15,15))
    for i in range(5):
        k=np.random.randint(0,x.shape[0]-1)
        m=x[k]
        t=l[k]
        pl.subplot(5,5,i+1)
        pl.title(str(t))
        pl.imshow(m)
    pl.show()

if __name__ == "__main__":
    (x_first,y_first),(x_second,y_second)=tfk.keras.datasets.mnist.load_data()
    tp(x_first,y_first)