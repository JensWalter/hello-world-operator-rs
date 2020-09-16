# hello-world-operator-rs
A kubernetes hello world operator in rust.

## Installing the operator

    kubectl apply -f https://raw.githubusercontent.com/JensWalter/hello-world-operator-rs/master/deploy/service-account.yaml

    kubectl apply -f https://raw.githubusercontent.com/JensWalter/hello-world-operator-rs/master/deploy/operator.yaml
    
    kubectl apply -f https://raw.githubusercontent.com/JensWalter/hello-world-operator-rs/master/deploy/crd-members.yaml

## Testing the opreator

To install a custom resource, you can use the samples provided from examples directory.

### example 1

    kubectl apply -f https://raw.githubusercontent.com/JensWalter/hello-world-operator-rs/master/examples/member1.yaml

This leads to the following output on the operator pod.

    welcome spongebob

### example 2

    kubectl apply -f https://raw.githubusercontent.com/JensWalter/hello-world-operator-rs/master/examples/member2.yaml

This leads to the following output on the operator pod.

    welcome patrick to the team blue



## Building the operator image from source

check out the git repo.

    docker build -t hello-world-operator-rs .

#### uploading image to a registry

    docker tag hello-world-operator-rs gcr.io/apimeister/hello-world-operator-rs
    docker push gcr.io/apimeister/hello-world-operator-rs

## Remove from the cluster


kubectl delete -f https://raw.githubusercontent.com/JensWalter/hello-world-operator-rs/master/deploy/service-account.yaml
kubectl delete -f https://raw.githubusercontent.com/JensWalter/hello-world-operator-rs/master/deploy/operator.yaml
kubectl delete -f https://raw.githubusercontent.com/JensWalter/hello-world-operator-rs/master/deploy/crd-person.yaml
