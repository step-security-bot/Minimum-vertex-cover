# Notes
## Description
The algorithm takes a lot of time to find the MVC of a graph. 
We are going to do a manual benchmark of some function of the BNB algorithm to see how it behaves.

## Benchmark

### brock200_4.clq

#### Iter 1
```text
Time spent in deg : 0.0752651657077773%
Time spent in clq : 0.8207294763378618%
Time spent in max deg : 0.0016415536823806804%
Time spent in copy : 0.0686178877645962%
Result : Minimum vertex cover for the "brock200_4.clq" graph = 183
The value is optimal (as long as the data is correct in the yaml file)
Time taken by the algorithm : 0min 33s 773ms 495µs
```


We see that the time spent in clq is very high. We are going to try to remove it.

#### Iter 2 (without clq)
```text
Time spent in deg : 0.3949931933067395%
Time spent in clq : 0.0005381172291803664%
Time spent in max deg : 0.011268925007748263%
Time spent in copy : 0.39869345675405066%
Result : Minimum vertex cover for the "brock200_4.clq" graph = 183
The value is optimal (as long as the data is correct in the yaml file)
Time taken by the algorithm : 0min 10s 125ms 323µs 
```

We divided the overall time by 3. We are going to try to remove the deg lb.

#### Iter 3 (without clq & deg)
```text
Time spent in deg : 0.0034830995525238313%
Time spent in clq : 0.0034754213195626487%
Time spent in max deg : 0.03762069103864573%
Time spent in copy : 0.5975289428466152%
Result : Minimum vertex cover for the "brock200_4.clq" graph = 183
The value is optimal (as long as the data is correct in the yaml file)
Time taken by the algorithm : 0min 30s 946ms 585µs 
```
We see that deg is usefully to cut some branches. 

### C125.9.clq

#### Iter 1
```text
Time spent in deg : 0.03834810766737626%
Time spent in clq : 0.9414832329604829%
Time spent in max deg : 0.0008765432061227506%
Time spent in copy : 0.013421898187853545%
Result : Minimum vertex cover for the "C125.9.clq" graph = 92
         The value is not optimal and the correct value is 91
         Time taken by the algorithm : 1min 2s 669ms 145µs 
```

#### Iter 2 (without clq)
```text
Time spent in deg : 0.6861045037663214%
Time spent in clq : 0.00046295114169564195%
Time spent in max deg : 0.015149646609213713%
Time spent in copy : 0.23040121792467946%
Result : Minimum vertex cover for the "C125.9.clq" graph = 92
         The value is not optimal and the correct value is 91
         Time taken by the algorithm : 0min 6s 654ms 565µs 
```