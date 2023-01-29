
install hypervisor

- skip if you have docker desktop
https://minikube.sigs.k8s.io/docs/drivers/hyperkit/

```
brew update
brew install hyperkit
```

install minikube, kubectl

```
brew install minikube

minikube

kubectl

# specify docker as driver, 
# https://medium.com/@seohee.sophie.kwon/how-to-run-a-minikube-on-apple-silicon-m1-8373c248d669
minikube start --driver=docker
```